// SPDX-License-Identifier: MPL-2.0

// Copyright (C) 2023  Soc Virnyl Estela

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
// SPDX-License-Identifier: MPL-2.0

#![deny(warnings)]
#![warn(unused_extern_crates)]
// Enable some groups of clippy lints.
#![deny(clippy::suspicious)]
#![deny(clippy::perf)]
// Specific lints to enforce.
#![warn(clippy::todo)]
#![deny(clippy::unimplemented)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::await_holding_lock)]
#![deny(clippy::needless_pass_by_value)]
#![deny(clippy::trivially_copy_pass_by_ref)]
#![deny(clippy::disallowed_types)]
#![deny(clippy::manual_let_else)]
#![allow(clippy::unreachable)]

use clap::Parser;
use glob::glob;
use obs_service_cargo::cli::{self, SrcDir, SrcTar};
use obs_service_cargo::consts::{VENDOR_EXAMPLE, VENDOR_PATH_PREFIX};
use obs_service_cargo::utils;
use obs_service_cargo::vendor::{self, vendor};

use std::ffi::OsStr;
use std::io;
use std::io::IsTerminal;
use std::path::{Path, PathBuf};
use terminfo::{capability as cap, Database};
use tracing_subscriber::EnvFilter;

#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn, Level};

// Create custom error type for processing

enum Src {
    Tar(SrcTar),
    Dir(SrcDir),
}

fn main() -> Result<(), io::Error> {
    let args = cli::VendorOpts::parse();
    let terminfodb = Database::from_env().map_err(|e| {
        error!(err = ?e, "Unable to access terminfo db. This is a bug!");
        io::Error::new(
            io::ErrorKind::Other,
            "Unable to access terminfo db. This is a bug!",
        )
    })?;

    let is_termcolorsupported = terminfodb.get::<cap::MaxColors>().is_some();
    let to_color = matches!(std::io::stdout().is_terminal(), true if {
        let coloroption = &args.color;
        match coloroption {
            clap::ColorChoice::Auto => is_termcolorsupported,
            clap::ColorChoice::Always => true,
            clap::ColorChoice::Never => false,
        }
    });

    let filter_layer = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .with_level(true)
        .with_ansi(to_color)
        .with_file(true)
        .with_line_number(true)
        .with_env_filter(filter_layer)
        .with_level(true)
        // Somehow pretty actually looks dank nasty
        // .pretty()
        .init();

    info!("🎢 Starting OBS Service Cargo Vendor.");
    debug!(?args);
    let tmpdir = tempfile::Builder::new()
        .prefix(VENDOR_PATH_PREFIX)
        .rand_bytes(8)
        .tempdir()
        .map_err(|e| {
            error!(err = ?e, "Unable to create temporary work directory.");
            e
        })?;

    let workdir: PathBuf = tmpdir.path().into();
    debug!("Created temporary working directory: {:?}", workdir);

    let src_type = match (&args.srcdir, &args.srctar) {
        (Some(srcdir), None) => Src::Dir(srcdir.clone()),
        (None, Some(srctar)) => Src::Tar(srctar.clone()),
        (Some(_), Some(_)) => {
            error!("Use only srcdir OR srctar - specifiying both is ambiguous");
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Use only srcdir OR srctar",
            ));
        }
        (None, None) => {
            error!("Must provide srcdir OR srctar");
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Must provide srcdir OR srctar",
            ));
        }
    };

    info!("Checking sources before vendor 🥡");

    let workdir = match src_type {
        Src::Dir(src) => {
            let basename = &src.srcdir.file_name().unwrap_or(src.srcdir.as_os_str());
            let newworkdir = &workdir.join(basename);
            debug!(?newworkdir);
            utils::copy_dir_all(&src.srcdir, newworkdir)?;
            // Only emit the message *after* we actually like ... checked.
            info!("Confirmed source is a directory: {:?}", src.srcdir);
            // Update the work dir
            newworkdir.clone()
        }
        Src::Tar(src) => {
            let glob_iter = match glob(&src.srctar.as_os_str().to_string_lossy()) {
                Ok(gi) => gi,
                Err(e) => {
                    error!(err = ?e, "Invalid srctar glob input");
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "Invalid srctar glob input",
                    ));
                }
            };

            let mut globs = glob_iter.into_iter().collect::<Vec<_>>();

            let matched_entry = match globs.len() {
                0 => {
                    error!("No files matched srctar glob input");
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "No files matched srctar glob input",
                    ));
                }
                1 => globs.remove(0),
                _ => {
                    error!("Multiple files matched srctar glob input");
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "Multiple files matched srctar glob input",
                    ));
                }
            };

            debug!(?matched_entry, "Globbed result");
            match matched_entry {
                // Balls.
                Ok(balls) => {
                    let newsrc = SrcTar { srctar: balls };
                    if newsrc.srctar.exists() {
                        newsrc.decompress(&workdir)?;
                        debug!(?newsrc.srctar);
                        debug!(?workdir);

                        // Only announce once we actually know.
                        info!("Confirmed source is a compressed tarball: {:?}", src.srctar);

                        // Leave the workdir as is.
                        workdir
                    } else {
                        error!(?newsrc, "Source does not exist based on path");
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidInput,
                            "Source does not exist based on path",
                        ));
                    }
                }
                Err(e) => {
                    error!(?e, "Got glob error");
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "Glob error"));
                }
            }
        }
    };

    debug!(?workdir);
    let target_file = OsStr::new("Cargo.toml");
    let mut possible_project_roots = match utils::find_file_multiples(&workdir, target_file) {
        Ok(v) => v,
        Err(err) => return Err(err),
    };

    possible_project_roots.sort_unstable_by_key(|path| path.components().count());
    debug!(?possible_project_roots);
    if let Some(prjdir) = possible_project_roots.first() {
        process_src(&args, prjdir, target_file)?;
    } else {
        warn!("Project has no root manifest!");
        if !args.cargotoml.is_empty() {
            vendor::cargotomls(&args, &workdir)?;
        } else {
            warn!("No subcrates to vendor.");
        }
    }

    info!("Vendor operation success! ❤️");
    info!("\n{}", VENDOR_EXAMPLE);

    // Remove temporary directory.
    tmpdir.close()?;

    info!("Successfully ran OBS Service Cargo Vendor 🥳");
    Ok(())
}

pub fn process_src(
    args: &cli::VendorOpts,
    prjdir: &Path,
    target_file: &OsStr,
) -> Result<(), io::Error> {
    info!("Guessed project root at uwu {}", prjdir.display());
    let pathtomanifest = prjdir.join(target_file);
    debug!(?pathtomanifest);
    if pathtomanifest.exists() {
        if let Ok(isworkspace) = utils::is_workspace(&pathtomanifest) {
            if isworkspace {
                info!(?pathtomanifest, "Project uses a workspace!");
            } else {
                info!(?pathtomanifest, "Project does not use a workspace!");
            };

            match vendor::has_dependencies(&pathtomanifest) {
                Ok(hasdeps) => {
                    if hasdeps && isworkspace {
                        info!("Workspace has dependencies!");
                        vendor(args, prjdir, None)?;
                    } else if hasdeps && !isworkspace {
                        info!("Non-workspace manifest has dependencies!");
                        vendor(args, prjdir, None)?;
                    } else if !hasdeps && isworkspace {
                        info!("Workspace has no global dependencies. May vendor dependencies from member crates.");
                        vendor(args, prjdir, None)?;
                    } else {
                        // This is what we call a "zero cost" abstraction.
                        info!("No dependencies, no need to vendor!");
                    };
                }
                Err(err) => return Err(err),
            };

            if args.cargotoml.is_empty() {
                info!(?args.cargotoml, "No subcrates to vendor!");
            } else {
                info!(?args.cargotoml, "Found subcrates to vendor!");
                vendor::cargotomls(args, prjdir)?;
            }
        }
    } else {
        warn!("Project does not have a manifest file at the root of the project!");
        if args.cargotoml.is_empty() {
            info!(?args.cargotoml, "No subcrates to vendor!");
        } else {
            info!(?args.cargotoml, "Found subcrates to vendor!");
            vendor::cargotomls(args, prjdir)?;
        }
    }
    Ok(())
}
