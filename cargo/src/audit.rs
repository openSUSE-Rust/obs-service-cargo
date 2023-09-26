// SPDX-License-Identifier: MPL-2.0

// Copyright (C) 2023  Soc Virnyl Estela

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::error::Error;
use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::{fmt, io};

use crate::cli::{Compression, Opts, Src};
use crate::consts::AUDIT_PATH_PREFIX;
use crate::consts::EXCLUDED_RUSTSECS;
use crate::services::{self, Services};
use crate::utils::cargo_command;

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
    pub lockfile: Vec<PathBuf>,
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

#[derive(Debug)]
pub struct AuditFailed {
    error: String,
    boxy: Box<dyn Error>,
}

impl Display for AuditFailed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = format!("{}. Got {}", self.error, self.boxy);
        write!(f, "{}", msg)
    }
}

impl AuditOpts {
    #[allow(dead_code)]
    pub fn new(
        self,
        lockfiles: Vec<PathBuf>,
        outdir: Option<PathBuf>,
        color: clap::ColorChoice,
    ) -> Self {
        Self {
            lockfile: lockfiles,
            outdir,
            color,
        }
    }

    pub fn generate_opts(self, p: &Path) -> io::Result<Vec<Opts>> {
        make_opts(p)
    }
}

impl Error for AuditFailed {}

pub trait Audit {
    // Run audit sets the workdir before running other stuff
    fn run_audit(self, opts: &Opts) -> Result<(), AuditFailed>;
    fn audit_vendored_tar(self, opts: &Opts, workdir: &Path) -> io::Result<()>;
    fn audit_sources(self, opts: &Opts, workdir: &Path) -> io::Result<()>;
    fn audit_additional_locks(self, audit_opts: &AuditOpts, workdir: &Path) -> io::Result<()>;
}

impl Audit for Src {
    fn run_audit(self, _opts: &Opts) -> Result<(), AuditFailed> {
        let tmpdir = match tempfile::Builder::new()
            .prefix(AUDIT_PATH_PREFIX)
            .rand_bytes(8)
            .tempdir()
        {
            Ok(t) => t,
            Err(err) => {
                error!("{}", err);
                return Err(AuditFailed {
                    error: "Failed to create temporary directory".to_string(),
                    boxy: err.into(),
                });
            }
        };

        let workdir: PathBuf = tmpdir.path().into();
        info!(?workdir, "Created working directory");
        // TODO check and audit vendor.tar.xz. Use globs
        // TODO check and audit src. See `make_opts`.
        // TODO check and audit multiple `lockfile` params.
        info!("Succesfully audited! 👀");
        Ok(())
    }

    fn audit_vendored_tar(self, _opts: &Opts, _workdir: &Path) -> io::Result<()> {
        Ok(())
    }

    fn audit_sources(self, _opts: &Opts, _workdir: &Path) -> io::Result<()> {
        Ok(())
    }

    fn audit_additional_locks(self, _audit_opts: &AuditOpts, _workdir: &Path) -> io::Result<()> {
        Ok(())
    }
}

#[allow(dead_code)]
fn audit_src() {}

#[allow(dead_code)]
fn audit_vendor_tarball() {}

pub fn cargo_audit(workdir: &Path, lockfiles: &[&Path]) -> io::Result<()> {
    let subcommand = "audit";
    let mut default_options: Vec<String> = Vec::new();
    let other_options = &[
        "--json",
        "-c",
        "never",
        "-D",
        "warnings",
        "-n",
        "-d",
        "/usr/share/cargo-audit-advisory-db",
    ];
    for advisory in EXCLUDED_RUSTSECS.iter() {
        let ignore: String = "--ignore".into();
        default_options.push(ignore);
        default_options.push(advisory.to_string());
    }
    default_options.append(&mut other_options.iter().map(|s| s.to_string()).collect());
    for lockfile in lockfiles {
        default_options.push(lockfile.to_string_lossy().to_string());
        match cargo_command(subcommand, &default_options, workdir) {
            Ok(ay) => {
                info!("{}", ay);
            }
            Err(err) => {
                error!(?err);
                return Err(io::Error::new(
                    io::ErrorKind::Interrupted,
                    "Got execution error. Failed to run command for audit",
                ));
            }
        }
        default_options.pop();
    }

    Ok(())
}

fn process_service_file(p: &Path) -> io::Result<services::Services> {
    Services::from_file(p)
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
