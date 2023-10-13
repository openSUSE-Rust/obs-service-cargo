// SPDX-License-Identifier: MPL-2.0

// Copyright (C) 2023  Soc Virnyl Estela

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::error::Error;
use std::ffi::OsStr;
use std::fmt::{self, Display};
use std::io;
use std::path::{Path, PathBuf};

use crate::consts::{GZ_MIME, SUPPORTED_MIME_TYPES, VENDOR_PATH_PREFIX, XZ_MIME, ZST_MIME};
use crate::utils;

use clap::{Parser, ValueEnum};
use infer;

#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn, Level};

#[derive(Parser, Debug)]
#[command(
    author,
    name = "cargo_vendor",
    version,
    about = "OBS Source Service to vendor all crates.io and dependencies for Rust project locally",
    after_long_help = "Set verbosity and tracing through `RUST_LOG` environmental variable e.g. `RUST_LOG=trace`

Bugs can be reported on GitHub: https://github.com/uncomfyhalomacro/obs-service-cargo_vendor-rs/issues",
    max_term_width = 120
)]
pub struct Opts {
    #[clap(flatten)]
    pub src: Src,
    #[arg(
        long,
        value_enum,
        default_value_t,
        help = "What compression algorithm to use."
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
}

impl AsRef<Opts> for Opts {
    #[inline]
    fn as_ref(&self) -> &Opts {
        self
    }
}

#[derive(ValueEnum, Default, Debug, Clone, Copy)]
pub enum Compression {
    Gz,
    Xz,
    #[default]
    Zst,
}

#[derive(Debug)]
pub enum SupportedFormat {
    Compressed(Compression, PathBuf),
    Dir(PathBuf),
}

impl Display for SupportedFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            SupportedFormat::Compressed(comp_type, src) => {
                format!("Compression: {}, Src: {}", comp_type, src.display())
            }
            SupportedFormat::Dir(src) => format!("Directory: {}", src.display()),
        };
        write!(f, "{}", msg)
    }
}

#[derive(Debug)]
pub struct UnsupportedFormat {
    pub ext: String,
}

impl Display for UnsupportedFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = format!("Expected one of the supported types. Got {}", self.ext);
        write!(f, "{}", msg)
    }
}

impl std::error::Error for UnsupportedFormat {}

impl Display for Compression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            Compression::Gz => "gz",
            Compression::Xz => "xz",
            Compression::Zst => "zst",
        };
        write!(f, "{}", msg)
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
    fn run_vendor(&self, opts: &Opts) -> Result<(), VendorFailed>;
}

pub fn decompress(comp_type: &Compression, outdir: &Path, src: &Path) -> io::Result<()> {
    match comp_type {
        Compression::Gz => utils::decompress::targz(outdir, src),
        Compression::Xz => utils::decompress::tarxz(outdir, src),
        Compression::Zst => utils::decompress::tarzst(outdir, src),
    }
}

impl Vendor for Src {
    fn is_supported(&self) -> Result<SupportedFormat, UnsupportedFormat> {
        if let Ok(actual_src) = utils::process_globs(&self.src) {
            debug!(?actual_src, "Source got from glob pattern");
            if actual_src.is_file() {
                match infer::get_from_path(&actual_src) {
                    Ok(kind) => match kind {
                        Some(known) => {
                            if SUPPORTED_MIME_TYPES.contains(&known.mime_type()) {
                                trace!(?known);
                                if known.mime_type().eq(GZ_MIME) {
                                    Ok(SupportedFormat::Compressed(Compression::Gz, actual_src))
                                } else if known.mime_type().eq(XZ_MIME) {
                                    Ok(SupportedFormat::Compressed(Compression::Xz, actual_src))
                                } else if known.mime_type().eq(ZST_MIME) {
                                    Ok(SupportedFormat::Compressed(Compression::Zst, actual_src))
                                } else {
                                    unreachable!()
                                }
                            } else {
                                Err(UnsupportedFormat {
                                    ext: known.mime_type().to_string(),
                                })
                            }
                        }
                        None => Err(UnsupportedFormat {
                            ext: "`File type is not known`".to_string(),
                        }),
                    },
                    Err(err) => {
                        error!(?err);
                        Err(UnsupportedFormat {
                            ext: "`Cannot read file`".to_string(),
                        })
                    }
                }
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

    fn run_vendor(&self, opts: &Opts) -> Result<(), VendorFailed> {
        let tmpdir = match tempfile::Builder::new()
            .prefix(VENDOR_PATH_PREFIX)
            .rand_bytes(8)
            .tempdir()
        {
            Ok(t) => t,
            Err(err) => {
                error!("{}", err);
                return Err(VendorFailed {
                    error: "Failed to create temporary directory".to_string(),
                    boxy: err.into(),
                });
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
                                            VendorFailed {
                                                error: "Failed to read directory".to_string(),
                                                boxy: err.into(),
                                            }
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
                                                    return Err(
                                                        VendorFailed {
                                                            error: "Tarball was extracted but got a file and not a possible top-level directory.".to_string(),
                                                            boxy: io::Error::new(io::ErrorKind::InvalidData, "Expected a directory, got a file").into()
                                                        }
                                                    );
                                                }
                                            }
                                            Err(err) => {
                                                error!(?err, "Failed to read directory entry");
                                                return Err(VendorFailed {
                                                    error: "Failed to read directory entry"
                                                        .to_string(),
                                                    boxy: err.into(),
                                                });
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
                                return Err(VendorFailed {
                                    error: "Failed to decompress source".to_string(),
                                    boxy: err.into(),
                                });
                            }
                        }
                    }
                    SupportedFormat::Dir(srcpath) => {
                        match utils::copy_dir_all(
                            &srcpath,
                            &workdir.join(srcpath.file_name().unwrap_or(srcpath.as_os_str())),
                        ) {
                            Ok(_) => {
                                let new_wdir = workdir
                                    .join(srcpath.file_name().unwrap_or(srcpath.as_os_str()));
                                debug!(?new_wdir);
                                new_wdir
                            }
                            Err(err) => {
                                return Err(VendorFailed {
                                    error: "Failed to copy source path".to_string(),
                                    boxy: err.into(),
                                })
                            }
                        }
                    }
                }
            }
            Err(err) => {
                error!(?err);
                return Err(VendorFailed {
                    error: format!("Vendor failed. {}", err),
                    boxy: err.into(),
                });
            }
        };

        debug!(?newworkdir, "Workdir updated!");

        let target_file = OsStr::new("Cargo.toml");
        match utils::process_src(opts, &newworkdir, target_file) {
            Ok(_) => {
                info!("🥳 ✨ Successfull ran OBS Service Cargo Vendor ✨");
            }
            Err(err) => {
                error!(?err);
                return Err(VendorFailed {
                    error: err.to_string(),
                    boxy: err.into(),
                });
            }
        };
        drop(newworkdir);
        match tmpdir.close() {
            Ok(_) => Ok(()),
            Err(err) => Err(VendorFailed {
                error: "Failed to close and remove temporary directory".to_string(),
                boxy: err.into(),
            }),
        }
    }
}

#[derive(Debug)]
pub struct VendorFailed {
    error: String,
    boxy: Box<dyn Error>,
}

impl Display for VendorFailed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = format!("{}. Got {}", self.error, self.boxy);
        write!(f, "{}", msg)
    }
}
impl Error for VendorFailed {}
