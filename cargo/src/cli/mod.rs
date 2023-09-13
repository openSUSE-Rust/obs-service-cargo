// SPDX-License-Identifier: MPL-2.0

// Copyright (C) 2023  Soc Virnyl Estela

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::vendor::utils::decompress;
use crate::vendor::utils::get_compression_type;
use crate::vendor::utils::UnsupportedExtError;

use clap::{Args, Parser, ValueEnum};
use std::fmt::{self, Display};
use std::io;
use std::path::{Path, PathBuf};

#[allow(unused_imports)]
use tracing::{debug, error, info, warn, Level};

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
pub struct VendorOpts {
    #[clap(flatten)]
    pub srctar: Option<SrcTar>,
    #[clap(flatten)]
    pub srcdir: Option<SrcDir>,
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

// TODO: move opts to correct modules.

#[derive(Parser, Debug)]
#[command(
    author,
    name = "cargo_audit",
    version,
    about = "OBS Source Service to audit all crates.io and dependencies in a Rust project for security issues",
    after_long_help = "Set verbosity and tracing through `RUST_LOG` environmental variable e.g. `RUST_LOG=trace`

Bugs can be reported on GitHub: https://github.com/uncomfyhalomacro/obs-service-cargo_vendor-rs/issues",
    max_term_width = 120
)]
pub struct AuditOpts {
    #[clap(flatten)]
    pub srctar: Option<SrcTar>,
    #[clap(flatten)]
    pub srcdir: Option<SrcDir>,
    #[arg(
        long,
        default_value = "auto",
        default_missing_value = "always",
        value_name = "WHEN",
        help = "Whether WHEN to color output or not"
    )]
    pub color: clap::ColorChoice,
}

impl AuditOpts {}

impl AsRef<VendorOpts> for VendorOpts {
    #[inline]
    fn as_ref(&self) -> &VendorOpts {
        self
    }
}

#[derive(Args, Debug, Clone)]
pub struct SrcTar {
    #[arg(
        long,
        help = "Where to find packed sources",
        conflicts_with = "srcdir",
        required = false
    )]
    pub srctar: PathBuf,
}

impl SrcTar {
    pub fn get_compression(&self) -> Result<Compression, UnsupportedExtError> {
        get_compression_type(&self.srctar)
    }

    pub fn decompress(&self, outdir: impl AsRef<Path>) -> Result<(), io::Error> {
        match self.get_compression() {
            Ok(comp) => match comp {
                Compression::Gz => decompress::targz(outdir.as_ref(), &self.srctar),
                Compression::Xz => decompress::tarxz(outdir.as_ref(), &self.srctar),
                Compression::Zst => decompress::tarzst(outdir.as_ref(), &self.srctar),
            },
            Err(err) => Err(io::Error::new(io::ErrorKind::Other, err)),
        }
    }
}

#[derive(Args, Debug, Clone)]
pub struct SrcDir {
    #[arg(
        long,
        help = "Where to find unpacked sources",
        conflicts_with = "srctar",
        required = false
    )]
    pub srcdir: PathBuf,
}

#[derive(ValueEnum, Default, Debug, Clone)]
pub enum Compression {
    Gz,
    Xz,
    #[default]
    Zst,
}

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
