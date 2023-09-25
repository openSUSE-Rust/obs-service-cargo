// SPDX-License-Identifier: MPL-2.0

// Copyright (C) 2023  Soc Virnyl Estela

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

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

use std::io;
use std::io::IsTerminal;

use clap::Parser;
use terminfo::{capability as cap, Database};
#[allow(unused_imports)]
use tracing::{debug, error, info, warn, Level};
use tracing_subscriber::EnvFilter;

use obs_service_cargo::audit::AuditOpts;

fn main() -> io::Result<()> {
    let args = AuditOpts::parse();

    let terminfodb = Database::from_env().map_err(|e| {
        error!(err = ?e, "Unable to access terminfo db. This is a bug!");
        io::Error::new(
            io::ErrorKind::Other,
            "Unable to access terminfo db. This is a bug!",
        )
    })?;

    let is_termcolorsupported = terminfodb.get::<cap::MaxColors>().is_some();
    let to_color = matches!(std::io::stdout().is_terminal(), true if {
        let coloroption = &args.color;
        match coloroption {
            clap::ColorChoice::Auto => is_termcolorsupported,
            clap::ColorChoice::Always => true,
            clap::ColorChoice::Never => false,
        }
    });

    let filter_layer = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .with_level(true)
        .with_ansi(to_color)
        .with_file(true)
        .with_line_number(true)
        .with_env_filter(filter_layer)
        .with_level(true)
        // Somehow pretty actually looks dank nasty
        // .pretty()
        .init();

    info!("🕵️ Starting OBS Service Cargo Audit.");
    debug!(?args);

    let service_file = std::env::current_dir()?.join("_service");
    let _opts = args.generate_opts(&service_file)?;

    // let svpath = std::path::PathBuf::from("./_service");
    // info!(?svpath);
    // let exml = Services::from_file(&svpath)?;
    // info!(?exml);
    // let services_to_opts = obs_service_cargo::audit::make_opts(&svpath);
    // info!(?services_to_opts);

    Ok(())
}
