// SPDX-License-Identifier: MPL-2.0

// Copyright (C) 2024 To all Contributors of this project listed in CONTRIBUTORS.md

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
// SPDX-License-Identifier: MPL-2.0

pub mod compress;
pub mod decompress;

use std::ffi::OsStr;
use std::fmt::{self, Debug, Display};
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use crate::cli::{Compression, Opts};
use crate::errors::OBSCargoError;
use crate::errors::OBSCargoErrorKind;
use crate::vendor::{self, generate_lockfile, vendor};

use crate::audit::{perform_cargo_audit, process_reports};

use glob::glob;
#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn, Level};

pub fn copy_dir_all(src: impl AsRef<Path>, dst: &Path) -> Result<(), io::Error> {
    debug!("Copying sources");
    debug!(?dst);
    fs::create_dir_all(dst)?;
    Ok(for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        trace!(?entry);
        trace!(?ty);
        if ty.is_dir() {
            trace!(?ty, "Is directory?");
            copy_dir_all(&entry.path(), &dst.join(&entry.file_name()))?;

        // Should we respect symlinks?
        // } else if ty.is_symlink() {
        //     debug!("Is symlink");
        //     let path = fs::read_link(&entry.path())?;
        //     let path = fs::canonicalize(&path).unwrap();
        //     debug!(?path);
        //     let pathfilename = path.file_name().unwrap_or(OsStr::new("."));
        //     if path.is_dir() {
        //         copy_dir_all(&path, &dst.join(pathfilename))?;
        //     } else {
        //         fs::copy(&path, &mut dst.join(pathfilename))?;
        //     }

        // Be pedantic or you get symlink error
        } else if ty.is_file() {
            trace!(?ty, "Is file?");
            fs::copy(&entry.path(), &mut dst.join(&entry.file_name()))?;
        };
    })
}

pub fn process_src(args: &Opts, prjdir: &Path) -> Result<(), OBSCargoError> {
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

    // Let's ensure the lockfiles are generated even if they don't exist
    // This guarantees that the dependencies used are properly recorded
    generate_lockfile(&first_manifest)?;
    manifest_files.iter().try_for_each(generate_lockfile)?;

    // Setup some common paths we'll use from here out.
    let outdir = args.outdir.to_owned();
    let cargo_config = prjdir.join(".cargo/config.toml");
    let vendor_dir = prjdir.join("vendor");
    let update = args.update;

    // This is all pre-processing, which is affected by the single/multi Cargo.toml
    // case. We do all this first.

    // Assume we have deps
    let mut hasdeps = true;

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

    // NOTE
    // Two things to know:
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

    for manifest_file in manifest_files.iter() {
        let manifest_f = manifest_file.canonicalize().map_err(|err| {
            error!("Failed to canonicalize path: {}", err);
            OBSCargoError::new(OBSCargoErrorKind::VendorError, err.to_string())
        })?;

        let lockfile_path = manifest_f.parent().map(|path_f| path_f.join("Cargo.lock"));
        if let Some(lockfile_p) = lockfile_path {
            if lockfile_p.exists() {
                cargo_locks.push(lockfile_p)
            };
        };
    }

    // NOTE
    // And then we check if the first manifest also has a lockfile.
    if let Some(custom_path) = first_manifest.parent() {
        let lockfilepath = custom_path.join("Cargo.lock");
        if lockfilepath.exists() {
            debug!("Path to first cargo lock: {}", lockfilepath.display());
            cargo_locks.push(lockfilepath);
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
        )?;

        // Finally, compress everything together.
        let compression: &Compression = &args.compression;
        debug!("Compression is of {}", &compression);

        let mut paths_to_archive: Vec<PathBuf> = vec![cargo_config, vendor_dir.clone()];

        paths_to_archive.append(&mut cargo_locks);

        debug!("All paths to archive {:#?}", paths_to_archive);

        if vendor_dir.exists() {
            vendor::compress(
                outdir,
                prjdir,
                &paths_to_archive,
                compression,
                args.tag.as_deref(),
            )?;
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

pub fn process_globs(src: &Path) -> io::Result<PathBuf> {
    let glob_iter = match glob(&src.as_os_str().to_string_lossy()) {
        Ok(gi) => {
            trace!(?gi);
            gi
        }
        Err(e) => {
            error!(err = ?e, "Invalid srctar glob input");
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid srctar glob input",
            ));
        }
    };

    let mut globs = glob_iter
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| {
            error!(?e, "glob error");
            io::Error::new(io::ErrorKind::InvalidInput, "Glob error")
        })?;

    // There can legitimately be multiple matching files. Generally this happens with
    // tar_scm where you have name-v1.tar and the service reruns and creates
    // name-v2.tar. In this case, we would error if we demand a single match, when what
    // we really need is to take the *latest*. Thankfully for us, versions in rpm
    // tar names tend to sort lexicographically, so we can just sort this list and
    // the last element is the newest. (ie v2 sorts after v1).

    globs.sort_unstable();

    if globs.len() > 1 {
        warn!("⚠️  Multiple files matched glob");
        for item in &globs {
            warn!("- {}", item.display());
        }
    }

    // Take the last item.
    globs
        .pop()
        .map(|item| {
            info!("🍿 Vendoring for src '{}'", item.display());
            item
        })
        .ok_or_else(|| {
            error!("No files/directories matched src glob input");
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "No files/directories matched src glob input",
            )
        })
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
