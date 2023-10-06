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

impl Opts {
    pub fn new(
        src: &Path,
        compression: Compression,
        tag: &str,
        cargotoml: Vec<PathBuf>,
        update: bool,
        outdir: &Path,
    ) -> Self {
        Self {
            src: Src::new(src),
            compression,
            tag: Some(tag.to_string()),
            cargotoml,
            update,
            outdir: outdir.into(),
            color: clap::ColorChoice::Auto,
        }
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
            if actual_src.is_file() {
                match infer::get_from_path(&actual_src) {
                    Ok(kind) => match kind {
                        Some(known) => {
                            if SUPPORTED_MIME_TYPES.contains(&known.mime_type()) {
                                trace!(?known);
                                if known.mime_type().eq(GZ_MIME) {
                                    Ok(SupportedFormat::Compressed(
                                        Compression::Gz,
                                        self.src.clone(),
                                    ))
                                } else if known.mime_type().eq(XZ_MIME) {
                                    Ok(SupportedFormat::Compressed(
                                        Compression::Xz,
                                        self.src.clone(),
                                    ))
                                } else if known.mime_type().eq(ZST_MIME) {
                                    Ok(SupportedFormat::Compressed(
                                        Compression::Zst,
                                        self.src.clone(),
                                    ))
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
                Ok(SupportedFormat::Dir(self.src.clone()))
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
        info!(?workdir, "Created working directory");

        // Return workdir here?
        let newworkdir = match self.is_supported() {
            Ok(format) => match format {
                SupportedFormat::Compressed(compression_type, srcpath) => {
                    match decompress(&compression_type, &workdir, &srcpath) {
                        Ok(_) => workdir,
                        Err(err) => {
                            return Err(VendorFailed {
                                error: "Failed to decompress source".to_string(),
                                boxy: err.into(),
                            });
                        }
                    }
                }
                SupportedFormat::Dir(srcpath) => match utils::copy_dir_all(srcpath, &workdir) {
                    Ok(_) => workdir,
                    Err(err) => {
                        return Err(VendorFailed {
                            error: "Failed to copy source path".to_string(),
                            boxy: err.into(),
                        })
                    }
                },
            },
            Err(err) => {
                error!(?err);
                return Err(VendorFailed {
                    error: format!("Vendor failed. {}", err),
                    boxy: err.into(),
                });
            }
        };

        info!(?newworkdir, "Workdir updated!");

        let target_file = OsStr::new("Cargo.toml");
        info!("Running cargo vendor");
        match utils::process_src(opts, &newworkdir, target_file) {
            Ok(_) => {
                info!("Successfull ran OBS Service Cargo Vendor 🥳");
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
