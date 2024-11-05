// SPDX-License-Identifier: MPL-2.0

// Copyright (C) 2024 To all Contributors of this project listed in
// CONTRIBUTORS.md

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{
    io,
    path::{Path, PathBuf},
    str::FromStr,
};

pub const EXCLUDED_RUSTSECS: &[&str] = &[
    // NOTE: These two are excluded because they are fundamentally
    // silly and can never be fixed.
    // https://rustsec.org/advisories/RUSTSEC-2020-0071.html
    // https://rustsec.org/advisories/RUSTSEC-2020-0159.html
    "RUSTSEC-2020-0071",
    "RUSTSEC-2020-0159",
];

pub const OPENSUSE_CARGO_AUDIT_DB: &str = "/usr/share/cargo-audit-advisory-db";

use rustsec::{
    advisory::Id,
    report::{Report, Settings as ReportSettings},
    Database, Error as RustsecError, ErrorKind as RustsecErrorKind, Lockfile,
};
#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn, Level};

pub fn process_reports(reports: Vec<Report>) -> Result<(), io::Error> {
    let mut passed = true;

    // Now actually analyse the report.
    for report in reports {
        if report.vulnerabilities.found {
            passed = false;

            if report.vulnerabilities.count == 1 {
                warn!("⚠️  {} vulnerability found.", report.vulnerabilities.count);
            } else {
                warn!(
                    "⚠️  {} vulnerabilities found.",
                    report.vulnerabilities.count
                );
            }

            for vuln in report.vulnerabilities.list {
                let score = vuln
                    .advisory
                    .cvss
                    .map(|base| base.score().value().to_string())
                    .unwrap_or_else(|| "unset".to_string());
                let id = vuln.advisory.id;
                let name = vuln.package.name;
                let version = vuln.package.version;

                let mut category = String::new();
                for cat in vuln.advisory.categories.iter() {
                    category.push_str(&cat.to_string());
                    category.push(' ');
                }

                warn!("- {id} {name} {version} - categories {category}- cvss {score}");
            }

            error!("⚠️  You must action these before submitting this package.");
        }
    }

    if passed {
        info!("🎉 Cargo audit passed!");
        Ok(())
    } else {
        error!(
			"🛑 Vulnerabilities found in application dependencies. These must be actioned to proceed \
			 with vendoring."
		);
        Err(io::Error::new(
			io::ErrorKind::Interrupted,
			"Vulnerabilities found in application dependencies. These must be actioned to proceed with \
			 vendoring."
				.to_string(),
		))
    }
}

pub fn perform_cargo_audit(
    lockfiles: &[impl AsRef<Path>],
    exclude_ids: &[impl AsRef<str>],
) -> Result<Vec<Report>, RustsecError> {
    // Setup our exclusions.
    let ignore = EXCLUDED_RUSTSECS
        .iter()
        .copied()
        .chain(exclude_ids.iter().map(|as_str| {
            let s = as_str.as_ref();
            info!("⚠️  Accepted risk - {}", s);
            s
        }))
        .map(Id::from_str)
        .collect::<Result<Vec<_>, _>>()?;

    let db_path: PathBuf = OPENSUSE_CARGO_AUDIT_DB.into();
    let database = Database::open(db_path.as_path())?;
    let report_settings = ReportSettings {
        ignore,
        ..Default::default()
    };

    lockfiles
        .iter()
        .map(|lockfile_ref| {
            let lockfile_path: &Path = lockfile_ref.as_ref();
            Lockfile::load(lockfile_path)
                .map(|lockfile| Report::generate(&database, &lockfile, &report_settings))
                .map_err(|cargo_lock_err| {
                    error!(?cargo_lock_err);
                    RustsecError::new(RustsecErrorKind::BadParam, &cargo_lock_err)
                })
        })
        .collect()
}
