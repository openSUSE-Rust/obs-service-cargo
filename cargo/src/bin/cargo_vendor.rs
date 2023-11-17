// SPDX-License-Identifier: MPL-2.0

// Copyright (C) 2023  Soc Virnyl Estela

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
// SPDX-License-Identifier: MPL-2.0

#![deny(warnings)]
#![warn(unused_extern_crates)]
// Enable some groups of clippy lints.
#![deny(clippy::suspicious)]
#![deny(clippy::perf)]
// Specific lints to enforce.
#![warn(clippy::todo)]
#![deny(clippy::unimplemented)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::await_holding_lock)]
#![deny(clippy::needless_pass_by_value)]
#![deny(clippy::trivially_copy_pass_by_ref)]
#![deny(clippy::disallowed_types)]
#![deny(clippy::manual_let_else)]
#![allow(clippy::unreachable)]

use clap::Parser;
use obs_service_cargo::cli::{self, Vendor};

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
    let to_color = matches!(std::io::stdout().is_terminal(), true if {
        let coloroption = &args.color;
        match coloroption {
            clap::ColorChoice::Auto => is_termcolorsupported,
            clap::ColorChoice::Always => true,
            clap::ColorChoice::Never => false,
        }
    });

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

    if args.tag.is_some() {
        error!("⚠️  tags are no longer supported for vendoring.");
        eprintln!(
            r#"
When you have multiple Cargo.toml's in a project, you can specify them with

    <param name=\"cargotoml\">first/Cargo.toml</param>"
    <param name=\"cargotoml\">second/Cargo.toml</param>"

This will create a single vendor.tar that will work with both projects.
"#
        );
    }

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

    match args.src.run_vendor(&args) {
        Ok(_) => Ok(()),
        Err(err) => {
            error!("{}", err);
            Err(err.into())
        }
    }
}
