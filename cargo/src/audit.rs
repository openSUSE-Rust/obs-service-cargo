// SPDX-License-Identifier: MPL-2.0

// Copyright (C) 2023  Soc Virnyl Estela

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::io;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use crate::consts::{EXCLUDED_RUSTSECS, OPENSUSE_CARGO_AUDIT_DB};

use rustsec::{
    advisory::Id, report::Report, report::Settings as ReportSettings, Database,
    Error as RustsecError, ErrorKind as RustsecErrorKind, Lockfile,
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
                error!("⚠️ {} vulnerability found.", report.vulnerabilities.count);
            } else {
                error!("⚠️ {} vulnerabilities found.", report.vulnerabilities.count);
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
                    category.push_str(" ");
                }

                warn!("- {id} {name} {version} - categories {category}- cvss {score}");
            }

            warn!("⚠️ You must action these before submitting this package.");
        }
    }

    if passed {
        info!("🎉 Cargo audit passed!");
        return Ok(());
    } else {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Vulnerabilities found in vendored dependencies.",
        ));
    }
}

pub fn perform_cargo_audit(
    lockfiles: &[impl AsRef<Path>],
    exclude_ids: &[impl AsRef<str>],
) -> Result<Vec<Report>, RustsecError> {
    // Setup our exclusions.
    let ignore = EXCLUDED_RUSTSECS
        .iter()
        .map(|s| *s)
        .chain(exclude_ids.iter().map(|as_str| {
            let s = as_str.as_ref();
            info!("⚠️  Accepted risk - {}", s);
            s
        }))
        .map(|id_str| Id::from_str(id_str))
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
