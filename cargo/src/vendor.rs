// SPDX-License-Identifier: MPL-2.0

// Copyright (C) 2023  Soc Virnyl Estela

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::fs;
use std::io::{self, Write};
use std::os::unix::prelude::OsStrExt;
use std::path::{Path, PathBuf};

use crate::cli::{Compression, Opts};
use crate::utils::cargo_command;
use crate::utils::compress;
use crate::vendor;

use crate::audit::{perform_cargo_audit, process_reports};

#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn, Level};

pub fn vendor(
    opts: impl AsRef<Opts>,
    prjdir: impl AsRef<Path>,
    vendorname: Option<&str>,
) -> io::Result<()> {
    // Setup

    let mut prjdir = prjdir.as_ref().to_path_buf();
    debug!(?prjdir);

    // Hack. This is to use the `current_dir` parameter of `std::process`.
    let mut manifest_path = prjdir.clone();
    manifest_path.push("Cargo.toml");
    debug!(?manifest_path);

    let mut cargo_lock_path = prjdir.clone();
    cargo_lock_path.push("Cargo.lock");
    debug!(?cargo_lock_path);

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

    // Update dependencies.

    if *update {
        info!("⏫ Updating dependencies before vendor");
        let mut update_options: Vec<&str> = vec!["-vv", "--manifest-path"];
        let update_manifest_path =
            unsafe { std::str::from_utf8_unchecked(manifest_path.as_os_str().as_bytes()) };
        update_options.push(update_manifest_path);
        cargo_command("update", &update_options, &prjdir).map_err(|e| {
            error!(err = %e);
            io::Error::new(io::ErrorKind::Other, "Unable to execute cargo")
        })?;
        info!("⏫ Successfully ran cargo update");
    } else {
        warn!("😥 Disabled update of dependencies. You should enable this for security updates.");
    };

    // Now that we have updated, we can run the Cargo-audit. we do this now to save bandwidth
    // and time if there are security issues that would otherwise be allowed.

    let cargolocks = vec![cargo_lock_path];

    let reports = perform_cargo_audit(&cargolocks, &opts.as_ref().i_accept_the_risk).map_err(
        |rustsec_err| {
            error!(?rustsec_err, "Unable to complete cargo audit");
            io::Error::new(io::ErrorKind::Other, "Unable to complete cargo audit")
        },
    )?;

    debug!(?reports);

    process_reports(reports)?;

    // Run the vendor

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
    info!("📦 Archiving vendored dependencies...");
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

    let cargolocks: Vec<PathBuf> = vec!["../Cargo.lock".into()];

    match compression {
        Compression::Gz => {
            let fullfilename_with_ext = format!("{}.tar.gz", fullfilename.to_string_lossy());
            outdir.push(&fullfilename_with_ext);
            if outdir.exists() {
                warn!(
                    replacing = ?outdir,
                    "🔦 Compressed tarball for vendor exists AND will be replaced."
                );
            }
            debug!("Compressed to {}", outdir.to_string_lossy());
            compress::targz("vendor", outdir, &prjdir, &cargolocks)?
        }
        Compression::Xz => {
            let fullfilename_with_ext = format!("{}.tar.xz", fullfilename.to_string_lossy());
            outdir.push(&fullfilename_with_ext);
            if outdir.exists() {
                warn!(
                    replacing = ?outdir,
                    "🔦 Compressed tarball for vendor exists AND will be replaced."
                );
            }
            debug!("Compressed to {}", outdir.to_string_lossy());
            compress::tarxz("vendor", outdir, &prjdir, &cargolocks)?
        }
        Compression::Zst => {
            let fullfilename_with_ext = format!("{}.tar.zst", fullfilename.to_string_lossy());
            outdir.push(&fullfilename_with_ext);
            if outdir.exists() {
                warn!(
                    replacing = ?outdir,
                    "🔦 Compressed tarball for vendor exists AND will be replaced."
                );
            }
            debug!("Compressed to {}", outdir.to_string_lossy());
            compress::tarzst("vendor", outdir, &prjdir, &cargolocks)?
        }
    };
    debug!("Finished creating {} compressed tarball", compression);

    Ok(())
}

pub fn is_workspace(src: &Path) -> io::Result<bool> {
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

pub fn has_dependencies(src: &Path) -> io::Result<bool> {
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

pub fn cargotomls(opts: impl AsRef<Opts>, prjdir: impl AsRef<Path>) -> io::Result<()> {
    info!("Vendoring additional manifest!");
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
                    info!(?pathtomanifest, "Additional manifest uses a workspace!");
                } else {
                    info!(
                        ?pathtomanifest,
                        "Additional manifest does not use a workspace!"
                    );
                };
                let prefix = format!("{}.vendor", cratename.to_string_lossy());
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
