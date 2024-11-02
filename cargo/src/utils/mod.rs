// SPDX-License-Identifier: MPL-2.0

// Copyright (C) 2024 To all Contributors of this project listed in CONTRIBUTORS.md

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
// SPDX-License-Identifier: MPL-2.0

use std::ffi::OsStr;
use std::fmt::{self, Debug, Display};
use std::path::Path;
use std::path::PathBuf;

use crate::cli::Opts;
use crate::errors::OBSCargoError;
use crate::errors::OBSCargoErrorKind;
use crate::vendor::{self, generate_lockfile, vendor};

use crate::audit::{perform_cargo_audit, process_reports};

use libroast::common::Compression;
use libroast::operations::cli::RoastArgs;
use libroast::operations::roast::roast_opts;
use libroast::utils::copy_dir_all;

#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn, Level};

pub fn process_src(args: &Opts, prjdir: &Path) -> Result<(), OBSCargoError> {
    let v_workdir = tempfile::Builder::new()
        .prefix(".vendor")
        .rand_bytes(12)
        .tempdir()
        .map_err(|err| {
            error!(?err);
            OBSCargoError::new(OBSCargoErrorKind::VendorError, err.to_string())
        })?;
    let vendor_workdir = v_workdir.path();
    let mut manifest_files: Vec<PathBuf> = if !args.cargotoml.is_empty() {
        debug!("Using manually specified Cargo.toml files.");
        debug!(?args.cargotoml);
        args.cargotoml.iter().map(|p| prjdir.join(p)).collect()
    } else {
        debug!("Assuming Cargo.toml in root of the projectdir");
        vec![prjdir.join("Cargo.toml")]
    };

    let Some(first_manifest) = manifest_files.pop() else {
        warn!("Project does not have a discovered manifest or configured paths to Cargo.toml");
        return Err(OBSCargoError::new(
            OBSCargoErrorKind::VendorError,
            "Project does not have a discovered manifest or configured paths to Cargo.toml"
                .to_string(),
        ));
    };

    debug!(?first_manifest);
    debug!(?manifest_files);

    // Setup some common paths we'll use from here out.
    let outdir = args.outdir.to_owned();
    let cargo_config = prjdir.join(".cargo/config.toml");
    let vendor_dir = prjdir.join("vendor");
    let update = args.update;

    // This is all pre-processing, which is affected by the single/multi Cargo.toml
    // case. We do all this first.

    // Assume we have deps
    let mut hasdeps = true;

    // NOTE Generate the lockfile first
    // NOTE Two things to know:
    // 1. If a manifest is not a workspace manifest, it's likely the lockfile
    // is in the directory of where the manifest is
    // 2. If a manifest is part or a member of a workspace manifest, then it's
    // likely that the lockfile is on the path of where the workspace manifest
    // is.
    //
    // So we just eagerly take all manifest paths from the parameters, and
    // just check if there are any lockfiles there.
    //
    // We canonicalize the path first so we won't get any gotchas if we are
    // looking for the parent path.
    //
    // And then we only accept if there is `Some` kind of value there and ignore
    // if there is `None`.
    let mut cargo_locks: Vec<PathBuf> = Vec::new();

    // Let's ensure the lockfiles are generated even if they don't exist
    // This guarantees that the dependencies used are properly recorded. We do this twice!
    for manifest_file in manifest_files.iter() {
        let manifest_f = manifest_file.canonicalize().map_err(|err| {
            error!("Failed to canonicalize path: {}", err);
            OBSCargoError::new(OBSCargoErrorKind::VendorError, err.to_string())
        })?;

        let lockfile_path = manifest_f.parent().map(|path_f| path_f.join("Cargo.lock"));
        if let Some(lockfile_p) = lockfile_path {
            if lockfile_p.exists() {
                debug!("Path to extra lockfile: {}", lockfile_p.display());
                cargo_locks.push(lockfile_p)
            } else {
                info!("Path to extra lockfile not found: {}", lockfile_p.display());
                if generate_lockfile(manifest_file).is_ok() {
                    info!(
                        "🔒 Cargo lockfile created for extra lockfile at path: {}",
                        lockfile_p.display()
                    );
                    cargo_locks.push(lockfile_p)
                } else {
                    debug!("Path didn't generate manifest: {}", lockfile_p.display());
                }
            };
        };
    }

    // NOTE: We check if the first manifest also has a lockfile.
    if let Some(custom_path) = first_manifest.parent() {
        let lockfilepath = custom_path.join("Cargo.lock");
        if lockfilepath.exists() {
            debug!("Path to first cargo lock: {}", lockfilepath.display());
            cargo_locks.push(lockfilepath);
        } else {
            debug!(
                "Path to first cargo lock not found: {}",
                lockfilepath.display()
            );
            if generate_lockfile(&first_manifest).is_ok() {
                info!(
                    "🔒 Cargo lockfile created for first lockfile at path: {}",
                    lockfilepath.display()
                );
                cargo_locks.push(lockfilepath);
            } else {
                debug!("Path didn't generate manifest: {}", lockfilepath.display());
            };
        };
    };

    // Now switch on the multi vs single case.
    if manifest_files.is_empty() {
        // Single file
        let isworkspace = vendor::is_workspace(&first_manifest)?;

        if isworkspace {
            info!("📚 Project uses a workspace!");
        } else {
            info!("📗 Project does not use a workspace!");
        };

        // It's a workspace, or single toml, so we can actually check this.
        // in multiple crates we can't.
        hasdeps = vendor::has_dependencies(&first_manifest)?;

        let should_vendor = if hasdeps && isworkspace {
            debug!("Workspace has dependencies!");
            true
        } else if hasdeps && !isworkspace {
            debug!("Non-workspace manifest has dependencies!");
            true
        } else if !hasdeps && isworkspace {
            debug!(
                "Workspace has no global dependencies. May vendor dependencies from member crates."
            );
            true
        } else {
            // This is what we call a "zero cost" abstraction.
            debug!("😌 No dependencies, no need to vendor!");
            false
        };

        // hasdeps should be equal now with should_vendor.
        hasdeps = should_vendor;

        if update {
            vendor::update(prjdir, &first_manifest)?
        } else {
            warn!(
                "😥 Disabled update of dependencies. You should enable this for security updates."
            );
        }
        // Okay, we are ready to go now.
    } else if update {
        warn!("⚠️ Unable to update when multiple Cargo.toml files are specified. Ignoring `update` parameter with value set to `{}`.", update);

        // Then we check if at least one of the manifest files contains a dependency.
        // We yolo it if it is a workspace.
        info!("Starting to check if additional manifest have dependencies.");
        hasdeps = manifest_files.iter().any(|manifest_file| {
            debug!(
                "⭕ Checking additional manifest if it contains a dependency: {}",
                manifest_file.display()
            );
            vendor::has_dependencies(manifest_file)
                .map_err(|err| {
                    error!(error = %err);
                    err
                })
                .is_ok_and(|istrue| istrue)
                || vendor::is_workspace(manifest_file)
                    .map_err(|err| {
                        error!(error = %err);
                        err
                    })
                    .is_ok_and(|istrue| istrue)
        });

        if hasdeps {
            info!("🗼 At least one of the provided manifests have dependencies or is at least a workspace.");
        };
    };

    info!("🔓 Rechecking lockfiles!");
    for manifest_file in manifest_files.iter() {
        let manifest_f = manifest_file.canonicalize().map_err(|err| {
            error!("Failed to canonicalize path: {}", err);
            OBSCargoError::new(OBSCargoErrorKind::VendorError, err.to_string())
        })?;

        let lockfile_path = manifest_f.parent().map(|path_f| path_f.join("Cargo.lock"));
        if let Some(lockfile_p) = lockfile_path {
            if lockfile_p.exists() {
                debug!("Path to extra lockfile: {}", lockfile_p.display());
                cargo_locks.push(lockfile_p)
            } else {
                info!("Path to extra lockfile not found: {}", lockfile_p.display());
                if generate_lockfile(manifest_file).is_ok() {
                    info!(
                        "🔒 Cargo lockfile created for extra lockfile at path: {}",
                        lockfile_p.display()
                    );
                    cargo_locks.push(lockfile_p)
                } else {
                    debug!("Path didn't generate manifest: {}", lockfile_p.display());
                }
            };
        };
    }

    // NOTE: We check if the first manifest also has a lockfile.
    if let Some(custom_path) = first_manifest.parent() {
        let lockfilepath = custom_path.join("Cargo.lock");
        if lockfilepath.exists() {
            debug!("Path to first cargo lock: {}", lockfilepath.display());
            cargo_locks.push(lockfilepath);
        } else {
            debug!(
                "Path to first cargo lock not found: {}",
                lockfilepath.display()
            );
            if generate_lockfile(&first_manifest).is_ok() {
                info!(
                    "🔒 Cargo lockfile created for first lockfile at path: {}",
                    lockfilepath.display()
                );
                cargo_locks.push(lockfilepath);
            } else {
                debug!("Path didn't generate manifest: {}", lockfilepath.display());
            };
        };
    };

    debug!("All cargo locks: {:?}", cargo_locks);

    // Audit the Cargo.lock file.
    let reports =
        perform_cargo_audit(&cargo_locks, &args.i_accept_the_risk).map_err(|rustsec_err| {
            error!(?rustsec_err, "Unable to complete cargo audit");
            OBSCargoError::new(
                OBSCargoErrorKind::AuditError,
                "Unable to complete cargo audit".to_string(),
            )
        })?;

    debug!(?reports);

    process_reports(reports)?;

    if hasdeps {
        vendor(
            prjdir,
            &cargo_config,
            &first_manifest,
            &manifest_files,
            args.filter,
            args.respect_lockfile,
            args.versioned_dirs,
        )?;

        // Finally, compress everything together.
        let compression: &Compression = &args.compression;
        debug!("Compression is of {}", &compression);

        let mut paths_to_archive: Vec<PathBuf> = vec![cargo_config, vendor_dir.clone()];

        paths_to_archive.append(&mut cargo_locks);

        debug!("All paths to archive {:#?}", paths_to_archive);

        if vendor_dir.exists() {
            let vendor_filename = match &args.tag {
                Some(suffix) => format!("vendor-{}", suffix),
                None => "vendor".to_string(),
            };
            let vendor_filename_with_extension = match &args.compression {
                Compression::Gz => format!("{}{}", &vendor_filename, ".tar.gz"),
                Compression::Xz => format!("{}{}", &vendor_filename, ".tar.xz"),
                Compression::Zst => format!("{}{}", &vendor_filename, ".tar.zst"),
                Compression::Bz2 => format!("{}{}", &vendor_filename, ".tar.bz"),
                Compression::Not => format!("{}{}", &vendor_filename, ".tar"),
            };
            let vendor_doppel = vendor_workdir.join(&vendor_filename);
            copy_dir_all(vendor_dir, &vendor_doppel).map_err(|err| {
                error!(?err);
                OBSCargoError::new(OBSCargoErrorKind::VendorError, err.to_string())
            })?;

            for p in paths_to_archive {
                let canon_p = p.canonicalize().unwrap_or(p.to_path_buf());
                let stripped_canon_p = canon_p
                    .strip_prefix(prjdir)
                    .unwrap_or(Path::new(canon_p.file_stem().unwrap_or_default()));
                let p_to_vendor_workdir = vendor_workdir.join(stripped_canon_p);
                let p_to_vendor_workdir_parent =
                    p_to_vendor_workdir.parent().unwrap_or(Path::new(""));
                std::fs::create_dir_all(p_to_vendor_workdir_parent).map_err(|err| {
                    error!(?err);
                    OBSCargoError::new(
                        OBSCargoErrorKind::VendorError,
                        "Failed to create a directory".to_string(),
                    )
                })?;
                if canon_p.is_file() {
                    std::fs::copy(canon_p, p_to_vendor_workdir).map_err(|err| {
                        error!(?err);
                        OBSCargoError::new(OBSCargoErrorKind::VendorError, err.to_string())
                    })?;
                } else if canon_p.is_dir() {
                    copy_dir_all(canon_p, &p_to_vendor_workdir).map_err(|err| {
                        error!(?err);
                        OBSCargoError::new(OBSCargoErrorKind::VendorError, err.to_string())
                    })?;
                };
            }

            let roast_args = RoastArgs {
                target: vendor_workdir.to_path_buf(),
                include: None,
                exclude: None,
                additional_paths: None,
                outfile: PathBuf::from(vendor_filename_with_extension),
                outdir: Some(outdir),
                preserve_root: false,
                reproducible: true,
                ignore_git: true,
                ignore_hidden: false,
            };
            roast_opts(&roast_args, false).map_err(|err| {
                error!(?err);
                OBSCargoError::new(OBSCargoErrorKind::VendorCompressionFailed, err.to_string())
            })?;
        } else {
            error!("Vendor dir does not exist! This is a bug!");
            return Err(OBSCargoError::new(
                OBSCargoErrorKind::VendorError,
                "Vendor directory not found when attempting to vendor.".to_string(),
            ));
        };
    } else {
        warn!("🔥 No dependencies for project were found, skipping vendoring. If you think this is an error, please check your configuration.");
        return Ok(());
    };

    // And we're golden!
    Ok(())
}

pub fn cargo_command<S: AsRef<OsStr>>(
    subcommand: &str,
    options: &[S],
    curdir: impl AsRef<Path>,
) -> Result<String, ExecutionError> {
    let cmd = std::process::Command::new("cargo")
        .arg(subcommand)
        .args(options.iter().map(|s| s.as_ref()))
        .current_dir(curdir.as_ref())
        .output()
        .map_err(|e| {
            error!(err = ?e, "Unable to build cargo command");
            ExecutionError {
                command: format!("cargo {}", subcommand),
                exit_code: Some(-1),
                stdoutput: "".to_string(),
            }
        })?;
    trace!(?cmd);
    let stdoutput = String::from_utf8_lossy(&cmd.stdout);
    let stderrput = String::from_utf8_lossy(&cmd.stderr);
    if !cmd.status.success() {
        error!(?stdoutput);
        error!(?stderrput);
        return Err(ExecutionError {
            command: format!("cargo {}", subcommand),
            exit_code: cmd.status.code(),
            stdoutput: stdoutput.to_string(),
        });
    };
    debug!(?stdoutput);
    debug!(?stderrput);
    // Return the output on success as this has the infor for .cargo/config
    Ok(stdoutput.to_string())
}

pub struct ExecutionError {
    pub command: String,
    pub exit_code: Option<i32>,
    pub stdoutput: String,
}

impl Debug for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = format!(
            "ExecutionError {{ command: `{}`, exit_code: `{}` }}",
            self.command,
            self.exit_code.unwrap_or(-1)
        );

        write!(f, "{}", msg)
    }
}

impl Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = format!(
            "Failed to run command `{}`. Has exit code `{}`. Standard Output Error: {}",
            self.command,
            self.exit_code.unwrap_or(-1),
            self.stdoutput
        );

        write!(f, "{}", msg)
    }
}
