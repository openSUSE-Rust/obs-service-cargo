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
use libroast::common::Compression;

use clap::Parser;
use libroast::decompress;

use libroast::operations::cli::RawArgs;
use libroast::operations::raw::raw_opts;
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
    #[arg(long, default_value_t = true, action = clap::ArgAction::Set, help = "Whether to use the `--versioned-dirs` flag of cargo-vendor.")]
    pub versioned_dirs: bool,
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

pub fn decompress(comp_type: &Compression, outdir: &Path, src: &Path) -> io::Result<()> {
    match comp_type {
        Compression::Gz => decompress::targz(outdir, src),
        Compression::Xz => decompress::tarxz(outdir, src),
        Compression::Zst => decompress::tarzst(outdir, src),
        Compression::Bz2 => decompress::tarbz2(outdir, src),
        Compression::Not => decompress::vanilla(outdir, src),
    }
}

impl Src {
    pub fn run_vendor(&self, opts: &Opts) -> Result<(), OBSCargoError> {
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

        let workdir = &tmpdir.path();
        debug!(?workdir, "Created working directory");
        let src_path = libroast::utils::process_globs(&self.src).map_err(|err| {
            error!(?err);
            OBSCargoError::new(OBSCargoErrorKind::VendorError, err.to_string())
        })?;

        if src_path.is_dir() {
            libroast::utils::copy_dir_all(&src_path, workdir).map_err(|err| {
                error!(?err);
                OBSCargoError::new(OBSCargoErrorKind::VendorError, err.to_string())
            })?;
        } else if src_path.is_file() {
            let raw_args = RawArgs {
                target: src_path,
                outdir: Some(workdir.to_path_buf()),
            };
            raw_opts(raw_args, false).map_err(|err| {
                error!(?err);
                OBSCargoError::new(OBSCargoErrorKind::VendorError, err.to_string())
            })?;
        }

        let setup_workdir = {
            let dirs: Vec<Result<std::fs::DirEntry, std::io::Error>> = std::fs::read_dir(workdir)
                .map_err(|err| {
                    error!(?err);
                    OBSCargoError::new(OBSCargoErrorKind::VendorError, err.to_string())
                })?
                .collect();
            debug!(?dirs, "List of files and directories of the workdir");
            if dirs.len() > 1 {
                debug!(?workdir);
                workdir.to_path_buf()
            } else {
                match dirs.into_iter().last() {
                    Some(p) => match p {
                        Ok(dir) => {
                            if dir.path().is_dir() {
                                debug!("{}", dir.path().display());
                                // NOTE: return new workdir
                                dir.path()
                            } else {
                                error!(?dir, "Tarball was extracted but got a file and not a possible top-level directory.");
                                return Err(OBSCargoError::new(
                                    OBSCargoErrorKind::VendorError,
                                    "No top-level directory found after tarball was extracted"
                                        .to_string(),
                                ));
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
        };

        debug!(?setup_workdir, "Workdir updated!");

        match crate::utils::process_src(opts, &setup_workdir) {
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
        tmpdir
            .close()
            .map_err(|err| OBSCargoError::new(OBSCargoErrorKind::VendorError, err.to_string()))
    }
}
