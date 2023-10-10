// SPDX-License-Identifier: MPL-2.0

// Copyright (C) 2023  Soc Virnyl Estela

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::error::Error;
use std::ffi::OsStr;
use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{fmt, io};

use crate::cli::{decompress, Compression, Opts, Src, Vendor};
use crate::consts::{AUDIT_PATH_PREFIX, EXCLUDED_RUSTSECS, OPENSUSE_CARGO_AUDIT_DB};
use crate::services::{self, Services};

use rustsec::{
    advisory::Id, report::Report, report::Settings as ReportSettings, Database,
    Error as RustsecError, ErrorKind as RustsecErrorKind, Lockfile,
};

use clap::Parser;
use glob;

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
#[derive(Parser, Debug, Clone)]
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
    fn run_audit(self, audit_opts: &AuditOpts) -> io::Result<()>;
    fn audit_vendored_tar(self) -> io::Result<()>;
    // fn audit_sources(self, audit_opts: &AuditOpts) -> io::Result<()>;
    // fn audit_additional_locks(self, audit_opts: &AuditOpts, workdir: &Path) -> io::Result<()>;
}

impl Audit for Src {
    fn run_audit(self, _audit_opts: &AuditOpts) -> io::Result<()> {
        // Should we clone or use Ref? Eh...
        // self.clone().audit_sources(audit_opts)?;
        self.clone().audit_vendored_tar()?;
        Ok(())
    }
    fn audit_vendored_tar(self) -> io::Result<()> {
        let tmpdir = match tempfile::Builder::new()
            .prefix(AUDIT_PATH_PREFIX)
            .rand_bytes(8)
            .tempdir()
        {
            Ok(t) => t,
            Err(err) => {
                error!("{}", err);
                return Err(err);
            }
        };

        let workdir: PathBuf = tmpdir.path().into();
        let current_path: PathBuf = std::env::current_dir()
            .map_err(|err| {
                error!(?err, "Unable to determinne working directory");
                err
            })?
            .join("*vendor.tar.*");

        let vendored_balls = match glob::glob(&current_path.to_string_lossy()) {
            Ok(blob) => {
                trace!(?blob);
                blob
            }
            Err(err) => {
                error!(?err, "Blob pattern error");
                return Err(io::Error::new(io::ErrorKind::Other, "Blob pattern error"));
            }
        };

        for ball in vendored_balls {
            trace!(?ball);
            match ball {
                Ok(fart) => {
                    trace!(?fart);
                    if fart.exists() && fart.is_file() {
                        // TODO: Process balls in temporary directory.
                        let vsauce = Src::new(&fart);
                        let newworkdir = match vsauce.is_supported() {
                            Ok(sauce) => match sauce {
                                crate::cli::SupportedFormat::Compressed(comp, ball_path) => {
                                    match decompress(&comp, &workdir, &ball_path) {
                                        Ok(_) => workdir,
                                        Err(err) => {
                                            error!(?err, "Failed to decompress source");
                                            return Err(err);
                                        }
                                    }
                                }
                                crate::cli::SupportedFormat::Dir(err) => {
                                    error!(?err, "No tarball should be a directory. This should be unreachable!");
                                    unreachable!();
                                }
                            },
                            Err(err) => {
                                error!(?err, "Format unsupported");
                                return Err(io::Error::new(
                                    io::ErrorKind::Unsupported,
                                    "Unsupported format, please check sources",
                                ));
                            }
                        };
                        let target_file = OsStr::new("Cargo.lock");
                        let lockfile_path = newworkdir.join(target_file);
                        if !lockfile_path.exists() {
                            error!(?lockfile_path, "File does not exist");
                            return Err(io::Error::new(
                                io::ErrorKind::NotFound,
                                "Expected lockfile. Found no such file.",
                            ));
                        } else if !lockfile_path.is_file() {
                            error!(
                                ?lockfile_path,
                                "Expected lockfile as a file, yet it's not a file."
                            );
                            return Err(io::Error::new(
                                io::ErrorKind::NotFound,
                                "Expected lockfile as file, yet it's not a file.",
                            ));
                        };
                        let reports =
                            perform_cargo_audit(&[&lockfile_path]).map_err(|rustsec_err| {
                                error!(?rustsec_err, "Unable to complete cargo audit");
                                io::Error::new(
                                    io::ErrorKind::Other,
                                    "Unable to complete cargo audit",
                                )
                            })?;

                        debug!(?reports);

                        let mut passed = true;

                        // Now actually analyse the report.
                        for report in reports {
                            if report.vulnerabilities.found {
                                passed = false;

                                if report.vulnerabilities.count == 1 {
                                    error!("{} vulnerability found.", report.vulnerabilities.count);
                                } else {
                                    error!(
                                        "{} vulnerabilities found.",
                                        report.vulnerabilities.count
                                    );
                                }

                                for vuln in report.vulnerabilities.list {
                                    let score = vuln
                                        .advisory
                                        .cvss
                                        .map(|base| base.score().value())
                                        .unwrap_or(0.0);
                                    let id = vuln.advisory.id;
                                    let name = vuln.package.name;
                                    let version = vuln.package.version;

                                    warn!("{id} {name} {version} - cvss {score}");
                                }

                                warn!("You must action these before submitting this package.");
                            }
                        }

                        if passed {
                            info!("Cargo audit passed! 🎉");
                            return Ok(());
                        } else {
                            return Err(io::Error::new(
                                io::ErrorKind::Other,
                                "Vulnerabilities found in vendored dependencies.",
                            ));
                        }
                    } else {
                        return Err(io::Error::new(
                            io::ErrorKind::NotFound,
                            "Expected a file. Found directory",
                        ));
                    };
                }
                Err(err) => {
                    error!(?err, "Glob error");
                    continue;
                }
            }
        }
        trace!("Dropping workdir 💧");
        drop(workdir);
        tmpdir.close()?;
        Ok(())
    }

    /*
    fn audit_sources(self, audit_opts: &AuditOpts) -> io::Result<()> {
        info!("Auditing project's root lockfile 🔐");
        let tmpdir = match tempfile::Builder::new()
            .prefix(AUDIT_PATH_PREFIX)
            .rand_bytes(8)
            .tempdir()
        {
            Ok(t) => t,
            Err(err) => {
                error!("{}", err);
                return Err(err);
            }
        };

        let workdir: PathBuf = tmpdir.path().into();
        if self.src.exists() {
            let newworkdir = match self.is_supported() {
                Ok(so) => match so {
                    crate::cli::SupportedFormat::Compressed(comp, src) => {
                        match decompress(&comp, &workdir, &src) {
                            Ok(_) => workdir,
                            Err(err) => {
                                error!(?err, "Failed to decompress source");
                                return Err(err);
                            }
                        }
                    }
                    crate::cli::SupportedFormat::Dir(dir) => {
                        match crate::utils::copy_dir_all(dir, &workdir) {
                            Ok(_) => workdir,
                            Err(err) => {
                                error!(?err, "Failed to copy source path to workdir");
                                return Err(err);
                            }
                        }
                    }
                },
                Err(err) => {
                    error!(?err, "Unsupported file format");
                    return Err(io::Error::new(
                        io::ErrorKind::Unsupported,
                        "Unsupported file format",
                    ));
                }
            };

            let target_file = OsStr::new("Cargo.lock");
            let lockfile_path = newworkdir.join(target_file);
            if !lockfile_path.exists() {
                // For projects such as s390-tools that do not have a root manifest,
                // therefore, lockfile isn't at root of the project as well
                warn!(?lockfile_path, "File does not exist");
            } else if !lockfile_path.is_file() {
                error!(
                    ?lockfile_path,
                    "Expected lockfile as a file, yet it's not a file."
                );
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "Expected lockfile as file, yet it's not a file.",
                ));
            } else {
                        let report = perform_cargo_audit(&[&lockfile_path])
                            .map_err(|rustsec_err| {
                                error!(?rustsec_err, "Unable to complete cargo audit");
                                io::Error::new(
                                    io::ErrorKind::Other,
                                    "Unable to complete cargo audit",
                                )
                            })?;

                        debug!(?report);

                        // Now actually analyse the report.
                        todo!();
            };
            self.audit_additional_locks(audit_opts, &newworkdir)?;
            Ok(())
        } else {
            error!(?self, "Source not found");
            Err(io::Error::new(io::ErrorKind::NotFound, "Source not found"))
        }
    }

    fn audit_additional_locks(self, audit_opts: &AuditOpts, workdir: &Path) -> io::Result<()> {
        info!("Auditing additional lockfiles 🔐");
        if audit_opts.lockfile.is_empty() {
            warn!("No additional lockfiles to audit 🕵️");
        } else {
            for lock in audit_opts.lockfile.iter() {
                let lockfile_path = workdir.join(lock);
                if !lockfile_path.exists() {
                    error!(?lockfile_path, "File does not exist");
                    return Err(io::Error::new(
                        io::ErrorKind::NotFound,
                        "Expected lockfile. Found no such file.",
                    ));
                } else if !lockfile_path.is_file() {
                    error!(
                        ?lockfile_path,
                        "Expected lockfile as a file, yet it's not a file."
                    );
                    return Err(io::Error::new(
                        io::ErrorKind::NotFound,
                        "Expected lockfile as file, yet it's not a file.",
                    ));
                };
            }
        }
        Ok(())
    }
    */
}

#[allow(dead_code)]
fn audit_src() {}

#[allow(dead_code)]
fn audit_vendor_tarball() {}

pub fn perform_cargo_audit(lockfiles: &[&Path]) -> Result<Vec<Report>, RustsecError> {
    // Setup our exclusions.
    let ignore = EXCLUDED_RUSTSECS
        .iter()
        .map(|id_str| Id::from_str(id_str))
        .collect::<Result<Vec<_>, _>>()?;

    let db_path: PathBuf = OPENSUSE_CARGO_AUDIT_DB.into();
    let database = Database::open(db_path.as_path())?;
    let report_settings = ReportSettings {
        ignore,
        ..Default::default()
    };

    lockfiles
        .iter()
        .map(|lockfile_path| {
            Lockfile::load(lockfile_path)
                .map(|lockfile| Report::generate(&database, &lockfile, &report_settings))
                .map_err(|cargo_lock_err| {
                    error!(?cargo_lock_err);
                    RustsecError::new(RustsecErrorKind::BadParam, &cargo_lock_err)
                })
        })
        .collect()
}

fn process_service_file(p: &Path) -> io::Result<services::Services> {
    Services::from_file(p)
}

pub fn make_opts(p: &Path) -> io::Result<Vec<Opts>> {
    let mut vicky: Vec<Opts> = Vec::new();

    let serv = process_service_file(p).map_err(|err| {
        error!(?err, "Unable to process `_service` file");
        err
    })?;

    match serv.service {
        Some(vices) => {
            if !vices.is_empty() {
                let a_public_market: Vec<&services::Service> = vices
                    .iter()
                    .filter(|v| v.name == Some("cargo_vendor".to_string()))
                    .collect();
                if a_public_market.is_empty() {
                    error!(
                        ?a_public_market,
                        "`cargo_vendor` service not defined in `_service`"
                    );
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "`cargo_vendor` service not defined in `_service`",
                    ));
                };
                for vendor in a_public_market.iter() {
                    if let Some(part) = &vendor.param {
                        let mut src = PathBuf::new();
                        let mut comp = Compression::Zst;
                        let mut cargotomls = Vec::new();
                        let mut update = true;
                        let filter = true;
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
                                    filter,
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
    }
}
