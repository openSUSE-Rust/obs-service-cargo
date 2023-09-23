// SPDX-License-Identifier: MPL-2.0

// Copyright (C) 2023  Soc Virnyl Estela

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::io;
use std::path::{Path, PathBuf};

use crate::cli::{Compression, Opts};
use crate::services;

use clap::Parser;
#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn, Level};

/// # How it works
///
/// This binary will try to run `cargo audit --file <pathtolockfile>`.
///
/// This binary will read `_service` file and searches for the service
/// that corresponds to `cargo_vendor` and attempts to audit them.
/// If `cargo_vendor` service exists, it will use the following params,
/// if they exist:
/// 1. `cargotoml`
/// 2. `src`, `srctar`, `srcdir`
///
/// ## `cargotoml`
///
/// The `cargotoml` path is used to check if there is also a lockfile to audit
/// in the path where it resides. Otherwise, it will attempt to regenerate
/// the lockfile.
///
/// ## `src` `srctar` `srcdir`
///
/// This will be used to check the lockfile of the sources and audit it.
///
/// ### `lockfiles`
///
/// In case additional lockfiles exist to audit from sources.
///
/// ## Vendored tarball
///
/// Since `cargo_vendor` generates a lockfile and also includes it
/// when generating the vendored tarball, we also check the lockfiles
/// in those contents as well.
///
#[derive(Parser, Debug)]
#[command(
    author,
    name = "cargo_vendor",
    version,
    about = "OBS Source Service to vendor all crates.io and dependencies for Rust project locally.",
    after_long_help = "Set verbosity and tracing through `RUST_LOG` environmental variable e.g. `RUST_LOG=trace`

Bugs can be reported on GitHub: https://github.com/uncomfyhalomacro/obs-service-cargo_vendor-rs/issues",
    max_term_width = 120
)]
pub struct AuditOpts {
    #[arg(long, help = "Where to find other lockfiles for auditing.")]
    pub lockfiles: Vec<PathBuf>,
    #[arg(long, help = "Dummy parameter. It's not used but OBS loves it.")]
    pub outdir: Option<PathBuf>,
    #[arg(
        long,
        default_value = "auto",
        default_missing_value = "always",
        value_name = "WHEN",
        help = "Whether WHEN to color output or not"
    )]
    pub color: clap::ColorChoice,
}

pub trait Audit {
    fn run_audit(self, opts: &AuditOpts) -> io::Result<()>;
    fn process_lockfiles(self) -> io::Result<()>;
}

impl Audit for AuditOpts {
    fn run_audit(self, _opts: &AuditOpts) -> io::Result<()> {
        Ok(())
    }

    fn process_lockfiles(self) -> io::Result<()> {
        Ok(())
    }
}

#[allow(dead_code)]
fn process_service_file(p: &Path) -> io::Result<services::Services> {
    services::Services::from_file(p)
}

pub fn make_opts(p: &Path) -> io::Result<Vec<Opts>> {
    let mut vicky: Vec<Opts> = Vec::new();
    match process_service_file(p) {
        Ok(serv) => match serv.service {
            Some(vices) => {
                if !vices.is_empty() {
                    let a_public_market: Vec<&services::Service> = vices
                        .iter()
                        .filter(|v| v.name == Some("cargo_vendor".to_string()))
                        .collect();
                    if a_public_market.is_empty() {
                        error!(?a_public_market, "Services are non-existent");
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Services are non-existent",
                        ));
                    };
                    for vendor in a_public_market.iter() {
                        if let Some(part) = &vendor.param {
                            let mut src = PathBuf::new();
                            let mut comp = Compression::Zst;
                            let mut cargotomls = Vec::new();
                            let mut update = true;
                            let outdir_ = std::env::current_dir()?;
                            for pa in part {
                                if let (Some(pname), Some(txt)) = (&pa.name, &pa.text) {
                                    if ["src", "srctar", "srcdir"].contains(&pname.as_str()) {
                                        src.push(txt);
                                    } else if pname == "compression" {
                                        match txt.as_str() {
                                            "gz" => {
                                                comp = Compression::Gz;
                                            }
                                            "xz" => {
                                                comp = Compression::Xz;
                                            }
                                            // Use default
                                            _ => comp = Compression::Zst,
                                        };
                                    } else if pname == "cargotoml" {
                                        let manifestpath = PathBuf::from(txt);
                                        cargotomls.push(manifestpath.clone());
                                    } else if pname == "update" {
                                        if let Ok(bully) = txt.trim().parse::<bool>() {
                                            update = bully;
                                        }
                                    };
                                    vicky.push(Opts::new(
                                        &src,
                                        comp,
                                        "",
                                        cargotomls.clone(),
                                        update,
                                        &outdir_,
                                    ));
                                    break;
                                };
                            }
                        };
                    }
                    Ok(vicky)
                } else {
                    error!(?vices, "Services are non-existent");
                    Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Services are non-existent",
                    ))
                }
            }
            None => {
                error!(?serv, "Services are non-existent");
                Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Services are non-existent",
                ))
            }
        },
        Err(err) => {
            error!(?err);
            Err(err)
        }
    }
}
