// SPDX-License-Identifier: MPL-2.0

// Copyright (C) 2024 To all Contributors of this project listed in CONTRIBUTORS.md

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
// SPDX-License-Identifier: MPL-2.0

use clap::Parser;
use obs_service_cargo::cli;

use std::io;
use std::io::IsTerminal;

use terminfo::{capability as cap, Database};
#[allow(unused_imports)]
use tracing::{debug, error, info, warn, Level};
use tracing_subscriber::EnvFilter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::Opts::parse();

    let terminfodb = Database::from_env().map_err(|e| {
        error!(err = ?e, "Unable to access terminfo db. This is a bug!");
        io::Error::new(
            io::ErrorKind::Other,
            "Unable to access terminfo db. This is a bug! Setting color option to false!",
        )
    });

    let is_termcolorsupported = match terminfodb {
        Ok(hasterminfodb) => hasterminfodb.get::<cap::MaxColors>().is_some(),
        Err(_) => false,
    };

    let to_color = std::io::stdout().is_terminal()
        && match &args.color {
            clap::ColorChoice::Auto => is_termcolorsupported,
            clap::ColorChoice::Always => true,
            clap::ColorChoice::Never => false,
        };

    let filter_layer = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let builder = tracing_subscriber::fmt()
        .with_level(true)
        .with_ansi(to_color)
        .with_env_filter(filter_layer)
        .with_level(true);

    let builder = if cfg!(debug_assertions) {
        builder.with_file(true).with_line_number(true)
    } else {
        builder
    };

    builder.init();

    info!("🎢 Starting OBS Service Cargo Vendor.");
    debug!(?args);

    warn!("⚠️  Cargo Vendor has been rewritten in rust!");
    eprintln!(
        r#"
This rewrite introduces some small changes to how vendoring functions for your package.

* cargo_config is no longer created - it's part of the vendor.tar now
    * You can safely remove lines related to cargo_config from your spec file

* multiple cargotoml files can be specified and share a single vendor.tar
    * If multiple cargo.toml files are present update does not work. This is a known
      limitation of the process

* cargo_audit is now part of cargo_vendor, meaning you don't have to configure it separately

"#
    );

    Ok(args.run_vendor().map_err(|err| {
        error!("{}", err);
        err
    })?)
}
