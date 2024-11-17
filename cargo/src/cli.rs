// SPDX-License-Identifier: MPL-2.0

// Copyright (C) 2024 To all Contributors of this project listed in CONTRIBUTORS.md

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::io;
use std::path::{Path, PathBuf};

use crate::consts::VENDOR_PATH_PREFIX;
use crate::registry::run_cargo_vendor_home_registry;
use crate::vendor::run_cargo_vendor;
use libroast::common::Compression;

use clap::{Args, Parser, ValueEnum};
use libroast::operations::cli::RawArgs;
use libroast::operations::raw::raw_opts;
use libroast::utils::copy_dir_all;
use libroast::{decompress, utils};

#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn, Level};

#[derive(Debug, Clone, ValueEnum, Default)]
pub enum Method {
    Registry,
    #[default]
    Vendor,
}

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
    #[arg(
        long,
        value_enum,
        default_value_t,
        help = "Whether to use vendor or the registry."
    )]
    pub method: Method,
    #[arg(
        long,
        visible_aliases = ["srctar", "srcdir", "target"],
        help = "Where to find sources. Source is either a directory or a source tarball AND cannot be both."
    )]
    pub src: PathBuf,
    #[arg(
        long,
        short = 'C',
        help = "Whether you want to manually set the root of the project. Useful with a combination with `--manifest-path` (aliased as `--cargotoml`) or `--no-root-manifest`."
    )]
    pub custom_root: Option<String>,
    #[arg(
		long,
		short = 'N',
		requires_if("registry", "method"),
		default_value_t = false,
		action = clap::ArgAction::Set,
		help = "Available only if `--method` is set to registry. If a project has no root manifest, this flag is useful for those situations to set the manifest path manually. Useful in combination with `--manifest-path` (aliased as `--cargotoml`) flag.")]
    pub no_root_manifest: bool,
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
    #[arg(long, help = "Other cargo manifest files to sync with vendor or registry. Behaviour between methods changes. Consult the documentation.", visible_aliases = ["cargotoml"])]
    pub manifest_path: Vec<PathBuf>,
    #[arg(long, default_value_t = true, action = clap::ArgAction::Set, help = "Update dependencies or not.")]
    pub update: bool,
    #[arg(
        long,
        help = "Where to output vendor.tar* and cargo_config if method is vendor and registry.tar* if method is registry. If using with `osc service`, this option is automatically appended."
    )]
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
        short = 'L',
        action = clap::ArgAction::Set,
        help = "Whether to respect Cargo.lock or lockfiles by passing the `--locked` flag.",
        default_value_t = false
    )]
    pub respect_lockfile: bool,
    #[arg(
        long,
        help = "A list of rustsec-id's to ignore. By setting this value, you acknowledge that this issue does not affect your package and you should be exempt from resolving it."
    )]
    pub i_accept_the_risk: Vec<String>,
    #[arg(
        long,
        help = "Set of specific crates to update. If not empty, it will set the global update flag to false. You can specify a valid version string by adding a `@` after the crate name e.g. `foo@1.2.3`. You can also do recursive updates of a crate by appending `recursive` to `@` e.g. `foo@recursive`. However, recursive can't be used with precise. You can specify a manifest path to update a package with `+` e.g. `foo@1.0+foo/better/Cargo.toml`. See `cargo help update` for info about how to update specific crates."
    )]
    pub update_crate: Vec<String>,
    #[clap(flatten)]
    pub vendor_specific_args: VendorArgs,
}

#[derive(Debug, Args)]
pub struct VendorArgs {
    #[arg(long,requires_if("vendor", "method"), default_value_t = false, action = clap::ArgAction::Set, help = "Available only if `--method` is set to vendor. EXPERIMENTAL: Reduce vendor-tarball size by filtering out non-Linux dependencies.")]
    pub filter: bool,
    #[arg(long, requires_if("vendor", "method"), default_value_t = true, action = clap::ArgAction::Set, help = "Available only if `--method` is set to vendor. Whether to use the `--versioned-dirs` flag of cargo-vendor.")]
    pub versioned_dirs: bool,
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

impl Opts {
    pub fn run_vendor(&mut self) -> io::Result<()> {
        debug!(?self);
        let tempdir_for_workdir = tempfile::Builder::new()
            .prefix(VENDOR_PATH_PREFIX)
            .rand_bytes(12)
            .tempdir()?;
        let workdir = &tempdir_for_workdir.path();
        debug!(?workdir);
        let target = utils::process_globs(&self.src)?;
        if target.is_dir() {
            copy_dir_all(&target, workdir)?;
        } else if target.is_file() && utils::is_supported_format(&target).is_ok() {
            let raw_args = RawArgs {
                target: target.to_path_buf(),
                outdir: Some(workdir.to_path_buf()),
            };
            raw_opts(raw_args, false)?;
        } else {
            return Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "Unsupported format found",
            ));
        }

        let setup_workdir = {
            let dirs: Vec<Result<std::fs::DirEntry, std::io::Error>> =
                std::fs::read_dir(workdir)?.collect();
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
                                error!(
                                                                ?dir,
                                                                "Tarball was extracted but got a file and not a possible top-level directory."
                                                        );
                                return Err(io::Error::new(
                                    io::ErrorKind::Interrupted,
                                    "No top-level directory found after tarball was extracted"
                                        .to_string(),
                                ));
                            }
                        }
                        Err(err) => {
                            error!(?err, "Failed to read directory entry");
                            return Err(err);
                        }
                    },
                    None => {
                        error!("This should be unreachable here");
                        unreachable!();
                    }
                }
            }
        };

        let custom_root = if let Some(custom_root) = &self.custom_root {
            info!(?custom_root, "ℹ️ Custom root is set.");
            setup_workdir.join(custom_root)
        } else {
            setup_workdir.to_path_buf()
        };

        // It won't make sense for update to be globally true while specifying to update a package
        if !&self.update_crate.is_empty() {
            warn!(?self.update_crate,
                "⚠️ Global update flag was set to false because specific crates to update are set!"
            );
            self.update = false;
        }

        if setup_workdir.exists() && setup_workdir.is_dir() {
            match &self.method {
                Method::Registry => {
                    run_cargo_vendor_home_registry(&setup_workdir, &custom_root, self)
                }
                Method::Vendor => run_cargo_vendor(&setup_workdir, &custom_root, self),
            }?;
        } else {
            let mut msg: String =
                "It seems that the setup workdir is not a directory or does not exist.".to_string();
            if self.custom_root.is_some() {
                msg.push_str(" Please check if your custom root has been setup properly!");
            } else {
                msg.push_str(" This seems to be a bug. Please file an issue at <https://github.com/openSUSE-Rust/obs-service-cargo/issues>.");
            }
            error!(msg);
            return Err(io::Error::new(io::ErrorKind::Other, msg));
        }
        info!("🌟 OBS Service Cargo finished.");
        info!("🧹 Cleaning up temporary directories...");
        tempdir_for_workdir.close()?;
        Ok(())
    }
}
