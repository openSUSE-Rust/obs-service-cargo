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

Bugs can be reported on GitHub: https://github.com/openSUSE/obs-service-cargo_vendor/issues",
    max_term_width = 120
)]
pub struct BulkUpdaterOpts {
    #[arg(
        long,
        default_value = "home:firstyear:branches",
        help = "Run bulk updater in this OBS project."
    )]
    basepath: PathBuf,
    #[arg(
        long,
        default_value_t = false,
        help = "Whether to yolo commit to the OBS branches."
    )]
    yolo: bool,
    #[arg(
        long,
        default_value_t = false,
        requires = "yolo",
        help = "Whether to findout by submitting requests for all committed branches. Requires yolo."
    )]
    findout: bool,
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
        use rayon::prelude::*;

        let out_packages: Vec<(PathBuf, io::Result<PathBuf>)> = self
            .packages
            .par_iter()
            .map(|package_name| -> (PathBuf, io::Result<PathBuf>) {
                (
                    package_name.to_path_buf(),
                    operations::checkout_or_update(&package_name.to_string_lossy(), &self.basepath),
                )
            })
            .collect();

        // Show the list of packages that we are not able to check out
        let _erred_out_packages = out_packages.par_iter().map(|(pkgname, result)| {
            if result.is_err() {
                tracing::error!(
                    "❌ Package {} failed to check out or update!",
                    pkgname.to_string_lossy()
                );
            }
        });

        // Then we get those that went successful
        let okay_checkout_packages: Vec<_> = out_packages
            .par_iter()
            .filter(|(_, result)| result.is_ok())
            .collect();

        let attempted_update_packages: Vec<_> = okay_checkout_packages
            .par_iter()
            .map(|(_old_package_path, result)| match result {
                Ok(new_package_path) => (
                    new_package_path,
                    operations::attempt_update(new_package_path, self.color),
                ),
                Err(_) => unreachable!(),
            })
            .collect();

        // Show the list of packages that we are not able to update
        let _failed_to_update_packages =
            attempted_update_packages
                .par_iter()
                .map(|(new_package_path, result)| {
                    if result.is_err() {
                        tracing::error!(
                            "❌ Package {} failed to update!",
                            new_package_path.to_string_lossy()
                        );
                    }
                });

        // We only need the package path since it will be reused anyway
        let updated_packages: Vec<_> = attempted_update_packages
            .par_iter()
            .filter_map(|(package_path, result)| {
                if result.is_ok() {
                    Some(package_path.to_path_buf())
                } else {
                    None
                }
            })
            .collect();

        let attempted_submit_packages: Vec<_> = updated_packages
            .par_iter()
            .map(|updated_package_path| {
                (
                    updated_package_path,
                    operations::attempt_submit(
                        updated_package_path,
                        &self.message,
                        self.yolo,
                        self.findout,
                    ),
                )
            })
            .collect();

        // Show the list of packages that we are not able to submit
        let _failed_to_submit_packages =
            attempted_submit_packages
                .par_iter()
                .map(|(package_path, result)| {
                    if result.is_err() {
                        tracing::error!(
                            "❌ Package {} failed to update!",
                            package_path.to_string_lossy()
                        );
                    }
                });

        Ok(())
    }
}
