// SPDX-License-Identifier: MPL-2.0

// Copyright (C) 2024 To all Contributors of this project listed in CONTRIBUTORS.md

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::io;
use std::path::{Path, PathBuf};

use crate::consts::VENDOR_PATH_PREFIX;
use crate::errors::OBSCargoError;
use crate::errors::OBSCargoErrorKind;
use crate::utils;
use libroast::common::Compression;
use libroast::common::{SupportedFormat, UnsupportedFormat};

use clap::Parser;
use libroast::decompress;

#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn, Level};

#[derive(Parser, Debug)]
#[command(
    author,
    name = "cargo_vendor",
    version,
    about = "OBS Source Service to vendor all crates.io and dependencies for Rust project locally",
    after_long_help = "Set verbosity and tracing through `RUST_LOG` environmental variable e.g. `RUST_LOG=trace`

Bugs can be reported on GitHub: https://github.com/openSUSE/obs-service-cargo_vendor/issues",
    max_term_width = 120
)]
pub struct Opts {
    #[clap(flatten)]
    pub src: Src,
    #[arg(
        long,
        value_enum,
        default_value_t,
        help = "What compression algorithm to use. Set to `not` if you just want a normal tarball with no compression."
    )]
    pub compression: Compression,
    #[arg(
        long,
        help = "Tag some files for multi-vendor and multi-cargo_config projects"
    )]
    pub tag: Option<String>,
    #[arg(long, help = "Other cargo manifest files to sync with during vendor")]
    pub cargotoml: Vec<PathBuf>,
    #[arg(long, default_value_t = true, action = clap::ArgAction::Set, help = "Update dependencies or not")]
    pub update: bool,
    #[arg(long, default_value_t = false, action = clap::ArgAction::Set, help = "EXPERIMENTAL: Reduce vendor-tarball size by filtering out non-Linux dependencies.")]
    pub filter: bool,
    #[arg(long, help = "Where to output vendor.tar* and cargo_config")]
    pub outdir: PathBuf,
    #[arg(
        long,
        default_value = "auto",
        default_missing_value = "always",
        value_name = "WHEN",
        help = "Whether WHEN to color output or not"
    )]
    pub color: clap::ColorChoice,

    #[arg(
        long,
        help = "A list of rustsec-id's to ignore. By setting this value, you acknowledge that this issue does not affect your package and you should be exempt from resolving it."
    )]
    pub i_accept_the_risk: Vec<String>,
    #[arg(long, default_value_t = true, action = clap::ArgAction::Set, help = "Respect lockfile or not if it exists. Otherwise, regenerate the lockfile and try to respect the lockfile.")]
    pub respect_lockfile: bool,
}

impl AsRef<Opts> for Opts {
    #[inline]
    fn as_ref(&self) -> &Opts {
        self
    }
}

#[derive(clap::Args, Debug, Clone)]
pub struct Src {
    #[arg(
        long,
        visible_aliases = ["srctar", "srcdir"],
        help = "Where to find sources. Source is either a directory or a source tarball AND cannot be both."
    )]
    pub src: PathBuf,
}

impl Src {
    pub fn new(p: &Path) -> Self {
        Self { src: p.into() }
    }
}

pub trait Vendor {
    fn is_supported(&self) -> Result<SupportedFormat, UnsupportedFormat>;
    fn run_vendor(&self, opts: &Opts) -> Result<(), OBSCargoError>;
}

pub fn decompress(comp_type: &Compression, outdir: &Path, src: &Path) -> io::Result<()> {
    match comp_type {
        Compression::Gz => decompress::targz(outdir, src),
        Compression::Xz => decompress::tarxz(outdir, src),
        Compression::Zst => decompress::tarzst(outdir, src),
        Compression::Bz2 => decompress::tarbz2(outdir, src),
        Compression::Not => decompress::vanilla(outdir, src),
    }
}

impl Vendor for Src {
    fn is_supported(&self) -> Result<SupportedFormat, UnsupportedFormat> {
        if let Ok(actual_src) = utils::process_globs(&self.src) {
            debug!(?actual_src, "Source got from glob pattern");
            if actual_src.is_file() {
                libroast::utils::is_supported_format(&actual_src)
            } else {
                Ok(SupportedFormat::Dir(actual_src))
            }
        } else {
            error!("Sources cannot be determined!");
            Err(UnsupportedFormat {
                ext: format!("unsupported source {}", &self.src.display()),
            })
        }
    }

    fn run_vendor(&self, opts: &Opts) -> Result<(), OBSCargoError> {
        let tmpdir = match tempfile::Builder::new()
            .prefix(VENDOR_PATH_PREFIX)
            .rand_bytes(8)
            .tempdir()
        {
            Ok(t) => t,
            Err(err) => {
                error!("{}", err);
                return Err(OBSCargoError::new(
                    OBSCargoErrorKind::VendorError,
                    "failed to create temporary directory for vendor process".to_string(),
                ));
            }
        };

        let workdir: PathBuf = tmpdir.path().into();
        debug!(?workdir, "Created working directory");

        // Return workdir here?
        let newworkdir: PathBuf = match self.is_supported() {
            Ok(format) => {
                match format {
                    SupportedFormat::Compressed(compression_type, srcpath) => {
                        match decompress(&compression_type, &workdir, &srcpath) {
                            Ok(_) => {
                                let dirs: Vec<Result<std::fs::DirEntry, std::io::Error>> =
                                    std::fs::read_dir(&workdir)
                                        .map_err(|err| {
                                            error!(?err, "Failed to read directory");
                                            OBSCargoError::new(
                                                OBSCargoErrorKind::VendorError,
                                                "failed to read directory".to_string(),
                                            )
                                        })?
                                        .collect();
                                trace!(?dirs, "List of files and directories of the workdir");
                                // If length is one, this means that the project has
                                // a top-level folder
                                if dirs.len() != 1 {
                                    debug!(?workdir);
                                    workdir
                                } else {
                                    match dirs.into_iter().last() {
                                        Some(p) => match p {
                                            Ok(dir) => {
                                                if dir.path().is_dir() {
                                                    debug!("{}", dir.path().display());
                                                    dir.path()
                                                } else {
                                                    error!(?dir, "Tarball was extracted but got a file and not a possible top-level directory.");
                                                    return Err(OBSCargoError::new(OBSCargoErrorKind::VendorError, "No top-level directory found after tarball was extracted".to_string()));
                                                }
                                            }
                                            Err(err) => {
                                                error!(?err, "Failed to read directory entry");
                                                return Err(OBSCargoError::new(
                                                    OBSCargoErrorKind::VendorError,
                                                    err.to_string(),
                                                ));
                                            }
                                        },
                                        None => {
                                            error!("This should be unreachable here");
                                            unreachable!();
                                        }
                                    }
                                }
                            }
                            Err(err) => {
                                return Err(OBSCargoError::new(
                                    OBSCargoErrorKind::VendorError,
                                    err.to_string(),
                                ));
                            }
                        }
                    }
                    SupportedFormat::Dir(srcpath) => match libroast::utils::copy_dir_all(
                        &srcpath,
                        &workdir.join(srcpath.file_name().unwrap_or(srcpath.as_os_str())),
                    ) {
                        Ok(_) => workdir.join(srcpath.file_name().unwrap_or(srcpath.as_os_str())),
                        Err(err) => {
                            return Err(OBSCargoError::new(
                                OBSCargoErrorKind::VendorError,
                                err.to_string(),
                            ))
                        }
                    },
                }
            }
            Err(err) => {
                error!(?err);
                return Err(OBSCargoError::new(
                    OBSCargoErrorKind::VendorError,
                    err.to_string(),
                ));
            }
        };

        debug!(?newworkdir, "Workdir updated!");

        match utils::process_src(opts, &newworkdir) {
            Ok(_) => {
                info!("🥳 ✨ Successfull ran OBS Service Cargo Vendor ✨");
            }
            Err(err) => {
                error!(?err);
                return Err(OBSCargoError::new(
                    OBSCargoErrorKind::VendorError,
                    err.to_string(),
                ));
            }
        };
        drop(newworkdir);
        tmpdir
            .close()
            .map_err(|err| OBSCargoError::new(OBSCargoErrorKind::VendorError, err.to_string()))
    }
}
