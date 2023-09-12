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

use obs_service_cargo::audit;
use obs_service_cargo::audit::Src;
use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;

struct AuditError {
    msg: String,
}

impl Debug for AuditError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Display for AuditError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for AuditError {}

fn main() -> Result<(), AuditError> {
    if let Some(arg) = std::env::args().nth(1) {
        let pathtofile = std::path::PathBuf::from(arg);
        if let Ok(this) = audit::read_service(pathtofile.as_path()) {
            // TODO: Use the values to find Cargo.lock
            // IMPORTANT: There are two ways to do this
            // 1. We either just get Cargo.lock from vendored tarball; or
            // 2. We will get Cargo.lock based on srctar and srcdir.

            // NOTE: As for method 1 for Cargo.lock, it's fairly easy to find it.
            // 1. Extract vendored tarball
            // 2. Find Cargo.lock

            // NOTE: As for method 2, this one is hard because we may have to copy
            // logic from cargo_vendor. But instead of finding `Cargo.toml`, we will look for
            // Cargo.lock

            // NOTE: But what about `cargotoml` parameter AND there is no Cargo.lock?
            // ANSWER: Well, we just run `cargo_vendor`. *Vendoring generates Cargo.lock*.
            // **We will just ask the user to do cargo_vendor first**.

            // WARN: Another thing to look out for if there is no presence of Cargo.lock.
            // This can be worked around by just running `cargo_vendor` since vendoring generates one.
            // But at least the user still gets the same vendored tarball *if they disabled updates*

            // NOTE: The full cargo audit command should use `-f` or `--file`. We do not need to copy
            // whole tarball or directory to a temporary directory.
            match this {
                Src::Tar(srctar) => {
                    println!("{:?}", srctar);
                }
                Src::Dir(srcdir) => {
                    println!("{:?}", srcdir);
                }
            }
            Ok(())
        } else {
            Err(AuditError {
                msg: "Yes".to_string(),
            })
        }
    } else {
        Err(AuditError {
            msg: "Yes".to_string(),
        })
    }
}
