// SPDX-License-Identifier: MPL-2.0

// Copyright (C) 2023  Soc Virnyl Estela

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{io, path::PathBuf};

use clap::Parser;
use tracing;

use crate::operations;

#[derive(Parser, Debug)]
#[command(
    author,
    name = "bulk_updater",
    about = "OBS Service Cargo Bulk Updater for Rust projects.",
    after_long_help = "Set verbosity and tracing through `RUST_LOG` environmental variable e.g. `RUST_LOG=trace`

Bugs can be reported on GitHub: https://github.com/uncomfyhalomacro/obs-service-cargo_vendor-rs/issues",
    max_term_width = 120
)]
pub struct BulkUpdaterOpts {
    #[arg(
        long,
        default_value = "home:firstyear:branches",
        help = "Custom basepath to run bulk updater."
    )]
    basepath: PathBuf,
    #[arg(long, default_value_t = false, help = "Yolo the bulk updates.")]
    yolo: bool,
    #[arg(
        long,
        default_value = "Automatic update of vendored dependencies",
        help = "Insert custom message for submitting updated package to OBS."
    )]
    message: String,
    #[arg(long, help = "List of Rust packages to be updated.")]
    packages: Vec<PathBuf>,
    #[arg(
        long,
        default_value = "auto",
        default_missing_value = "always",
        value_name = "WHEN",
        help = "Whether WHEN to color output or not"
    )]
    pub color: clap::ColorChoice,
}

impl BulkUpdaterOpts {
    pub fn run(self) -> io::Result<()> {
        let mut pkgpaths: Vec<PathBuf> = Vec::new();
        for pkgname in self.packages.iter() {
            tracing::info!(
                "⏫ Checkout or update in progress for {}",
                pkgname.to_string_lossy()
            );
            pkgpaths.push(operations::checkout_or_update(
                &pkgname.to_string_lossy(),
                &self.basepath,
            )?);
            tracing::info!("🥳 Updated {}", pkgname.to_string_lossy());
        }
        for pkgpath in pkgpaths.iter() {
            tracing::info!(
                "🔼 Attempting to update in progress at {}",
                pkgpath.to_string_lossy()
            );
            let updated_pkgpath = operations::attempt_update(pkgpath)?;
            tracing::info!("✅ Updated package at {}", pkgpath.to_string_lossy());
            tracing::info!(
                "📤 Submitting package in progress at {}",
                updated_pkgpath.to_string_lossy()
            );
            let submitted_pkgpath = operations::attempt_submit(pkgpath, &self.message, self.yolo)?;

            tracing::info!(
                "📥 Submitted package at {}",
                submitted_pkgpath.to_string_lossy()
            );
        }
        Ok(())
    }
}
