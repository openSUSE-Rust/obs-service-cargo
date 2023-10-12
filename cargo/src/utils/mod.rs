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

use crate::cli::Opts;
use crate::vendor;
use crate::vendor::vendor;

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
pub fn process_src(args: &Opts, prjdir: &Path, target_file: &OsStr) -> io::Result<()> {
    let pathtomanifest = prjdir.join(target_file);

    if !args.cargotoml.is_empty() {
        info!(?args.cargotoml, "Found crates to vendor!");
        // I think i can just reuse process src instead of invoking this?
        vendor::cargotomls(args, prjdir)
    } else if pathtomanifest.exists() {
        if let Ok(isworkspace) = vendor::is_workspace(&pathtomanifest) {
            debug!(?pathtomanifest);
            if isworkspace {
                info!("📚 Project uses a workspace!");
            } else {
                info!("📗 Project does not use a workspace!");
            };

            match vendor::has_dependencies(&pathtomanifest) {
                Ok(hasdeps) => {
                    if hasdeps && isworkspace {
                        debug!("Workspace has dependencies!");
                        vendor(args, prjdir, None)?;
                    } else if hasdeps && !isworkspace {
                        debug!("Non-workspace manifest has dependencies!");
                        vendor(args, prjdir, None)?;
                    } else if !hasdeps && isworkspace {
                        debug!("Workspace has no global dependencies. May vendor dependencies from member crates.");
                        vendor(args, prjdir, None)?;
                    } else {
                        // This is what we call a "zero cost" abstraction.
                        info!("😌 No dependencies, no need to vendor!");
                    };
                }
                Err(err) => return Err(err),
            };
        }

        Ok(())
    } else {
        debug!(?pathtomanifest);
        warn!("Project does not have a manifest or configured paths to Cargo.toml");
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Project does not have a manifest or configured paths to Cargo.toml",
        ))
    }
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

pub fn cargo_command<S: AsRef<str>>(
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
