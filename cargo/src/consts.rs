// SPDX-License-Identifier: MPL-2.0

// Copyright (C) 2023  Soc Virnyl Estela

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub const VENDOR_PATH_PREFIX: &str = ".obs-service-cargo-vendor";
pub const AUDIT_PATH_PREFIX: &str = ".obs-service-cargo-audit";
pub const VENDOR_EXAMPLE: &str =
    "Examples of how to modify your spec file to use vendored libraries can be found online:

https://en.opensuse.org/Packaging_Rust_Software#Creating_the_Package

WARNING: To avoid cargo install rebuilding the binary in the install stage
         all environment variables must be the same as in the build stage.
";

pub const XZ_EXTS: &[&str] = &["xz"];
pub const ZST_EXTS: &[&str] = &["zstd", "zst"];
pub const GZ_EXTS: &[&str] = &["gz", "gzip"];
pub const XZ_MIME: &str = "application/x-xz";
pub const ZST_MIME: &str = "application/zstd";
pub const GZ_MIME: &str = "application/gzip";
pub const SUPPORTED_MIME_TYPES: &[&str] = &[XZ_MIME, ZST_MIME, GZ_MIME];
pub const EXCLUDED_RUSTSECS: &[&str] = &[
    // NOTE: These two are excluded because they are fundamentally
    // silly and can never be fixed.
    // https://rustsec.org/advisories/RUSTSEC-2020-0071.html
    // https://rustsec.org/advisories/RUSTSEC-2020-0159.html
    "RUSTSEC-2020-0071",
    "RUSTSEC-2020-0159",
];

pub const OPENSUSE_CARGO_AUDIT_DB: &str = "/usr/share/cargo-audit-advisory-db";
