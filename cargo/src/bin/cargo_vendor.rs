// SPDX-License-Identifier: MPL-2.0

// Copyright (C) 2023  Soc Virnyl Estela

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
// SPDX-License-Identifier: MPL-2.0

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

    info!("🎢 Starting OBS Service Cargo Vendor.");
    debug!(?args);

    match args.src.is_supported() {
        Ok(ay) => {
            info!(?ay, "Source is supported");
        }
        Err(err) => {
            error!("{}", err);
            return Err(io::Error::new(io::ErrorKind::InvalidInput, err).into());
        }
    };
    match args.src.run_vendor(&args) {
        Ok(_) => Ok(()),
        Err(err) => {
            error!("{}", err);
            Err(err.into())
        }
    }
}
