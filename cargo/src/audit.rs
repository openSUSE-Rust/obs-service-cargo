// SPDX-License-Identifier: MPL-2.0

// Copyright (C) 2023  Soc Virnyl Estela

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// use std::error::Error;
// use std::fmt;
use std::io;
use std::path::PathBuf;
// use std::process;

use clap::Parser;

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
pub struct AuditOpts {
    #[clap(flatten)]
    src: AuditSrc,
    #[arg(long, help = "Where to find other lockfiles for auditing.")]
    lockfiles: Option<Vec<PathBuf>>,
}

#[derive(clap::Args, Debug, Clone)]
pub struct AuditSrc {
    #[arg(
        long,
        visible_aliases = ["srctar", "srcdir"],
        help = "Where to find sources. Source is either a directory or a source tarball AND cannot be both."
    )]
    // NOTE If `None`, check `_service`
    src: Option<PathBuf>,
}

// TODO: Replace some of the return types with a Custom Error
pub trait Audit {
    // RATIONALE: Running this command should be have two states
    // 1. With src option
    // 2. Without src option
    // If 2, read the `_service` file.
    fn run_audit(&self, opts: &AuditOpts) -> io::Result<()>;
}
