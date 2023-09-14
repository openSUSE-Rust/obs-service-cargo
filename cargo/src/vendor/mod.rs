// SPDX-License-Identifier: MPL-2.0

// Copyright (C) 2023  Soc Virnyl Estela

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::cli::{Compression, VendorOpts};
use crate::consts::{GZ_EXTS, GZ_MIME, SUPPORTED_MIME_TYPES, XZ_EXTS, XZ_MIME, ZST_EXTS, ZST_MIME};
use crate::vendor;

use infer;
use std::error::Error;
use std::fmt::Debug;
use std::fmt::{self, Display};
use std::fs::{self};
use std::io::{self, Write};
use std::os::unix::prelude::OsStrExt;
use std::path::Path;

use crate::utils::compress;

#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn, Level};

fn cargo_command(
    subcommand: &str,
    options: &[&str],
    curdir: impl AsRef<Path>,
) -> Result<String, ExecutionError> {
    let cmd = std::process::Command::new("cargo")
        .arg(subcommand)
        .args(options)
        .current_dir(curdir.as_ref())
        .output()
        .map_err(|e| {
            error!(err = ?e, "Unable to build cargo command");
            ExecutionError {
                command: format!("cargo {}", subcommand),
                exit_code: None,
            }
        })?;
    trace!(?cmd);
    let stdoutput = String::from_utf8_lossy(&cmd.stdout).to_string();
    if !cmd.status.success() {
        return Err(ExecutionError {
            command: format!("cargo {}", subcommand),
            exit_code: cmd.status.code(),
        });
    };
    Ok(stdoutput)
}

pub struct ExecutionError {
    pub command: String,
    pub exit_code: Option<i32>,
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
            "Failed to run command `{}`. Has exit code `{}`",
            self.command,
            self.exit_code.unwrap_or(-1)
        );

        write!(f, "{}", msg)
    }
}
pub fn vendor(
    opts: impl AsRef<VendorOpts>,
    prjdir: impl AsRef<Path>,
    vendorname: Option<&str>,
) -> Result<(), io::Error> {
    let mut prjdir = prjdir.as_ref().to_path_buf();
    debug!(?prjdir);
    // Hack. This is to use the `current_dir` parameter of `std::process`.
    let mut manifest_path = prjdir.clone();
    manifest_path.push("Cargo.toml");
    info!(?manifest_path);
    let update = &opts.as_ref().update;
    let mut outdir = opts.as_ref().outdir.to_owned();
    let fullfilename = vendorname.unwrap_or("vendor");
    let fullfilename = Path::new(fullfilename)
        .file_name()
        .unwrap_or(Path::new(fullfilename).as_os_str());
    let mut cargo_config = String::new();
    if fullfilename.to_string_lossy() == "vendor" {
        cargo_config.push_str("cargo_config");
    } else {
        let withprefix = format!("{}_cargo_config", fullfilename.to_string_lossy());
        cargo_config.push_str(&withprefix);
    };
    let cargo_config = Path::new(&cargo_config)
        .file_name()
        .unwrap_or(Path::new(&cargo_config).as_os_str());

    if *update {
        info!("Updating dependencies before vendor");
        let mut update_options: Vec<&str> = vec!["-vv", "--manifest-path"];
        let update_manifest_path =
            unsafe { std::str::from_utf8_unchecked(manifest_path.as_os_str().as_bytes()) };
        update_options.push(update_manifest_path);
        cargo_command("update", &update_options, &prjdir).map_err(|e| {
            error!(err = %e);
            io::Error::new(io::ErrorKind::Other, "Unable to execute cargo")
        })?;
        info!("Successfully ran cargo update ❤️");
    } else {
        warn!("Disabled update of dependencies. You may reenable it for security updates.");
    };

    let mut vendor_options: Vec<&str> = vec!["-vv", "--manifest-path"];
    let vendor_manifest_path =
        unsafe { std::str::from_utf8_unchecked(manifest_path.as_os_str().as_bytes()) };
    vendor_options.push(vendor_manifest_path);
    debug!(?vendor_options);
    let cargo_vendor_output = cargo_command("vendor", &vendor_options, &prjdir).map_err(|e| {
        error!(err = %e);
        io::Error::new(io::ErrorKind::Other, "Unable to execute cargo")
    })?;
    debug!(?outdir);
    let mut cargo_config_outdir = fs::File::create(outdir.join(cargo_config))?;
    cargo_config_outdir.write_all(cargo_vendor_output.as_bytes())?;
    info!("Proceeding to create compressed archive of vendored deps...");
    prjdir.push("vendor/");
    let compression: &Compression = &opts.as_ref().compression;
    debug!("Compression is of {}", &compression);

    // RATIONALE: We copy Cargo.lock by default, updated or not updated
    // `../` relative to `vendor/` directory.
    // CONSIDERATIONS:
    // Maybe in the future we can check if Cargo.toml points to a workspace
    // using the `toml` crate. For now, we aggressively just copy `../Cargo.lock`
    // relative to vendor directory if it exists. Even if it is a workspace,
    // it will still copy the project's `Cargo.lock` because we still run
    // `vendor` anyway starting at the root of the project where the lockfile resides.
    // NOTE: 1. The members in that workspace still requires that root lockfile.
    // NOTE: 2. Members of that workspace cannot generate their own lockfiles.
    // NOTE: 3. If they are not members, we slap that file into their own compressed vendored
    //          tarball
    let cargolock: Vec<&str> = vec!["../Cargo.lock"];
    match compression {
        Compression::Gz => {
            let fullfilename_with_ext = format!("{}.tar.gz", fullfilename.to_string_lossy());
            outdir.push(&fullfilename_with_ext);
            if outdir.exists() {
                warn!(
                    ?outdir,
                    "Compressed tarball for vendor exists. Please manually check sources 🔦"
                );
            }
            debug!("Compressed to {}", outdir.to_string_lossy());
            compress::targz("vendor", outdir, &prjdir, &cargolock)?
        }
        Compression::Xz => {
            let fullfilename_with_ext = format!("{}.tar.xz", fullfilename.to_string_lossy());
            outdir.push(&fullfilename_with_ext);
            if outdir.exists() {
                warn!(
                    ?outdir,
                    "Compressed tarball for vendor exists. Please manually check sources 🔦"
                );
            }
            debug!("Compressed to {}", outdir.to_string_lossy());
            compress::tarxz("vendor", outdir, &prjdir, &cargolock)?
        }
        Compression::Zst => {
            let fullfilename_with_ext = format!("{}.tar.zst", fullfilename.to_string_lossy());
            outdir.push(&fullfilename_with_ext);
            if outdir.exists() {
                warn!(
                    ?outdir,
                    "Compressed tarball for vendor exists. Please manually check sources 🔦"
                );
            }
            debug!("Compressed to {}", outdir.to_string_lossy());
            compress::tarzst("vendor", outdir, &prjdir, &cargolock)?
        }
    };
    info!("Finished creating {} compressed tarball", compression);
    Ok(())
}

#[derive(Debug)]
pub struct UnsupportedExtError {
    pub ext: Option<String>,
}

impl fmt::Display for UnsupportedExtError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match &self.ext {
            None => "No extension found for file. Please check if file has an extension or if it is actually a file.".to_string(),
            Some(err) => format!("{} is unsupported. If you think this is incorrect, please open an issue at https://github.com/uncomfyhalomacro/obs-service-cargo_vendor-rs/issues.", err)
        };
        write!(f, "{}", &msg)
    }
}

impl Error for UnsupportedExtError {}

pub fn get_compression_type(file: &Path) -> Result<Compression, UnsupportedExtError> {
    if file.is_file() {
        let info = infer::get_from_path(file).map_err(|e| {
            error!(err = ?e, "Unable to determine file information");
            UnsupportedExtError { ext: None }
        })?;

        let extension = match file.extension() {
            Some(ext) => unsafe { std::str::from_utf8_unchecked(ext.as_bytes()) },
            None => "unknown extension",
        };
        let mimetype = match info {
            Some(ext) => ext.mime_type(),
            None => "unknown mime type",
        };
        if !SUPPORTED_MIME_TYPES.contains(&mimetype) {
            error!(?mimetype);
            Err(UnsupportedExtError {
                ext: Some(mimetype.to_string()),
            })
        } else {
            match mimetype {
                XZ_MIME => {
                    if XZ_EXTS.contains(&extension) {
                        warn!("File has the correct supported extension {}", extension);
                    } else {
                        warn!("File has an incorrect extension: {}. Make sure it's the right compression AND extension to avoid confusion", extension);
                    };
                    Ok(Compression::Xz)
                }
                GZ_MIME => {
                    if GZ_EXTS.contains(&extension) {
                        warn!("File has the correct supported extension {}", extension);
                    } else {
                        warn!("File has an incorrect extension: {}. Make sure it's the right compression AND extension to avoid confusion", extension);
                    };
                    Ok(Compression::Gz)
                }
                ZST_MIME => {
                    if ZST_EXTS.contains(&extension) {
                        warn!("File has the correct supported extension {}", extension);
                    } else {
                        warn!("File has an incorrect extension: {}. Make sure it's the right compression AND extension to avoid confusion", extension);
                    };
                    Ok(Compression::Zst)
                }
                _ => unreachable!(),
            }
        }
    } else {
        let err = Err(UnsupportedExtError {
            ext: Some("Directory".to_string()),
        });
        error!(?err);
        err
    }
}

pub fn is_workspace(src: &Path) -> Result<bool, io::Error> {
    if let Ok(manifest) = fs::read_to_string(src) {
        if let Ok(manifest_data) = toml::from_str::<toml::Value>(&manifest) {
            if manifest_data.get("workspace").is_some() {
                return Ok(true);
            } else {
                return Ok(false);
            };
        };
    }
    return Err(io::Error::new(
        io::ErrorKind::NotFound,
        src.to_string_lossy(),
    ));
}

pub fn has_dependencies(src: &Path) -> Result<bool, io::Error> {
    if let Ok(manifest) = fs::read_to_string(src) {
        if let Ok(manifest_data) = toml::from_str::<toml::Value>(&manifest) {
            if manifest_data.get("dependencies").is_some()
                || manifest_data.get("dev-dependencies").is_some()
            {
                return Ok(true);
            } else {
                return Ok(false);
            };
        };
    }
    return Err(io::Error::new(
        io::ErrorKind::NotFound,
        src.to_string_lossy(),
    ));
}

pub fn cargotomls(opts: impl AsRef<VendorOpts>, prjdir: impl AsRef<Path>) -> Result<(), io::Error> {
    info!("Vendoring separate crate!");
    let cargotomls = &opts.as_ref().cargotoml.to_owned();
    trace!(?cargotomls);
    let prjdir = prjdir.as_ref().to_path_buf();

    for subcrate in cargotomls {
        let pathtomanifest = prjdir.join(subcrate);

        // Just return the original subcrate name.
        let manifestparent = subcrate.parent().unwrap_or(subcrate);
        let cratename = manifestparent
            .file_name()
            .unwrap_or(manifestparent.as_os_str());
        if pathtomanifest.exists() {
            if let Ok(isworkspace) = is_workspace(&pathtomanifest) {
                if isworkspace {
                    info!(?pathtomanifest, "Subcrate uses a workspace!");
                } else {
                    info!(?pathtomanifest, "Subcrate does not use a workspace!");
                };
                let prefix = format!("{}_vendor", cratename.to_string_lossy());
                let subprjdir = pathtomanifest
                    .parent()
                    .unwrap_or(prjdir.join(subcrate).as_path())
                    .to_path_buf();

                match vendor::has_dependencies(&pathtomanifest) {
                    Ok(hasdeps) => {
                        if hasdeps && isworkspace {
                            info!("Workspace has dependencies!");
                            vendor(&opts, &subprjdir, Some(&prefix))?;
                        } else if hasdeps && !isworkspace {
                            info!("Non-workspace manifest has dependencies!");
                            vendor(&opts, &subprjdir, Some(&prefix))?;
                        } else if !hasdeps && isworkspace {
                            info!("Workspace has no global dependencies. May vendor dependencies from member crates.");
                            vendor(&opts, &subprjdir, Some(&prefix))?;
                        } else {
                            // This is what we call a "zero cost" abstraction.
                            info!("No dependencies, no need to vendor!");
                        };
                    }
                    Err(err) => return Err(err),
                };
            };
        };
    }

    Ok(())
}
