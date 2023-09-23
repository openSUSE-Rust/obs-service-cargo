// SPDX-License-Identifier: MPL-2.0

// Copyright (C) 2023  Soc Virnyl Estela

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::io;
use std::path::PathBuf;

use clap::Parser;

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

/// TODO: Replace some of the return types with a Custom Error
pub trait Audit {
    fn run_audit(self, opts: &AuditOpts) -> io::Result<()>;

    fn process_src(self) -> io::Result<AuditOpts>;

    fn process_lockfiles(self) -> io::Result<()>;
}
