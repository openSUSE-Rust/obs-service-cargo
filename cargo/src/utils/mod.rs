// SPDX-License-Identifier: MPL-2.0

// Copyright (C) 2023  Soc Virnyl Estela

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
use crate::vendor::{self, vendor};

use crate::audit::{perform_cargo_audit, process_reports};

use glob::glob;
#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn, Level};

pub fn copy_dir_all(src: impl AsRef<Path>, dst: &Path) -> Result<(), io::Error> {
    debug!("Copying sources");
    debug!(?dst);
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
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
    }
    Ok(())
}

pub fn process_src(args: &Opts, prjdir: &Path) -> Result<(), OBSCargoError> {
    let mut manifest_files: Vec<PathBuf> = if !args.cargotoml.is_empty() {
        debug!("Using manually specified Cargo.toml files.");
        debug!(?args.cargotoml);
        args.cargotoml.iter().map(|p| p.into()).collect()
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
    let cargo_lock = prjdir.join("Cargo.lock");
    let cargo_config = prjdir.join("cargo_config");
    let vendor_dir = prjdir.join("vendor");
    let update = args.update;

    // This is all pre-processing, which is affected by the single/multi Cargo.toml
    // case. We do all this first.

    // Now switch on the multi vs single case.
    if manifest_files.is_empty() {
        // Single file
        let isworkspace = vendor::is_workspace(&first_manifest)?;

        if isworkspace {
            info!("📚 Project uses a workspace!");
        } else {
            info!("📗 Project does not use a workspace!");
        };

        let hasdeps = vendor::has_dependencies(&first_manifest)?;

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

        if !should_vendor {
            warn!("🔥 No dependencies for project were found, skipping vendoring. If you think this is an error, please check your configuration.");
            return Ok(());
        }

        if update {
            vendor::update(prjdir, &first_manifest)?
        } else {
            warn!(
                "😥 Disabled update of dependencies. You should enable this for security updates."
            );
        }
        // Okay, we are ready to go now.
    } else if update {
        warn!("⚠️  Unable to update when multiple Cargo.toml files are specified.");
        return Err(OBSCargoError::new(
            OBSCargoErrorKind::VendorError,
            "Unable to update when multiple Cargo.toml files are specified.".to_string(),
        ));
    }

    // Audit the Cargo.lock file.
    let reports =
        perform_cargo_audit(&[&cargo_lock], &args.i_accept_the_risk).map_err(|rustsec_err| {
            error!(?rustsec_err, "Unable to complete cargo audit");
            OBSCargoError::new(
                OBSCargoErrorKind::AuditError,
                "Unable to complete cargo audit".to_string(),
            )
        })?;

    debug!(?reports);

    process_reports(reports)?;

    vendor(prjdir, &cargo_config, &first_manifest, &manifest_files)?;

    // Finally, compress everything together.
    let compression: &Compression = &args.compression;
    debug!("Compression is of {}", &compression);

    let paths_to_archive: [&Path; 3] = [
        cargo_config.as_ref(),
        cargo_lock.as_ref(),
        vendor_dir.as_ref(),
    ];

    if vendor_dir.exists()
        && (vendor::has_dependencies(&first_manifest)? || vendor::is_workspace(&first_manifest)?)
    {
        vendor::compress(outdir, prjdir, &paths_to_archive, compression)?;
    } else {
        info!("😌 No dependencies, no need to vendor!");
    }

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
    // TODO ExecutionError should also have error output as String :)
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
