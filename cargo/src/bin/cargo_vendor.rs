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
use obs_service_cargo::consts::{PREFIX, VENDOR_EXAMPLE};
use obs_service_cargo::vendor::utils;

use std::io;
use std::io::IsTerminal;
use std::path::PathBuf;
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
    let args = cli::Opts::parse();
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
        .prefix(PREFIX)
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
    match utils::get_project_root(&workdir) {
        Ok(prjdir) => {
            // Addressed limitations of get_project_root
            let pathtomanifest = prjdir.join("Cargo.toml");
            if pathtomanifest.exists() {
                // Again, announce once we actually confirm the details.
                debug!("Guessed project root at {:?}", prjdir);
                if let Ok(isworkspace) = utils::is_workspace(&pathtomanifest) {
                    if isworkspace {
                        info!("Project uses a workspace.");
                        if utils::has_dependencies(&pathtomanifest).unwrap_or(false) {
                            info!("Workspace has global dependencies!");
                        } else {
                            info!(
                                "No global dependencies! May vendor dependencies of member crates"
                            );
                        };
                    } else {
                        // What is actually the need for the manual check? What's
                        // actionable here?
                        info!("Project is not a workspace. Please check manually! 🫂");
                        if utils::has_dependencies(&pathtomanifest).unwrap_or(false) {
                            info!("Project has dependencies!");
                        } else {
                            // This is what we call a "zero cost" abstraction.
                            info!("No dependencies, no need to vendor!");
                        };
                    };
                };

                utils::vendor(&args, &prjdir, None)?;
                if !args.cargotoml.is_empty() {
                    // Should this be here? If there are cargo.toml's listed, then
                    // it probably means that there are just a "list of crates" and we can't
                    // use the workspace manifest. For example s390-tools has neither a
                    // workspace NOR a project root. It's just 4 crates. So I think that
                    // here we actually need to not touch cargotomls.
                    //
                    // Consider it like this - you either have cargotomls listed because you
                    // want to exactly tell us where they are.
                    //
                    // OR
                    //
                    // You want us to guess and find it for you.
                    //
                    // Does that make sense?
                    info!("Subcrates to vendor found!");
                    utils::cargotomls(&args, &prjdir)?;
                } else {
                    info!("No subcrates to vendor!");
                };
            } else {
                warn!("This is not a rust project");
                warn!("Use the start of the root of the project to your subcrate instead!");
                // fallback to workdir
                utils::cargotomls(&args, &workdir)?;
            }
        }
        Err(err) => return Err(err),
    };

    info!("Vendor operation success! ❤️");
    info!("\n{}", VENDOR_EXAMPLE);

    // Remove temporary directory.
    tmpdir.close()?;

    info!("Successfully ran OBS Service Cargo Vendor 🥳");
    Ok(())
}
