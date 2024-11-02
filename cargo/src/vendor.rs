// SPDX-License-Identifier: MPL-2.0

// Copyright (C) 2024 To all Contributors of this project listed in CONTRIBUTORS.md

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::BTreeMap;
use std::ffi::OsString;
use std::fs;
use std::io::Write;
use std::path::Path;

use crate::errors::OBSCargoError;
use crate::errors::OBSCargoErrorKind;
use crate::utils::cargo_command;

use serde::Deserialize;
use serde::Serialize;

#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn, Level};

pub fn update(
    prjdir: impl AsRef<Path>,
    manifest_path: impl AsRef<Path>,
) -> Result<(), OBSCargoError> {
    info!("⏫ Updating dependencies before vendor");
    let update_options: Vec<OsString> = vec![
        "-vv".into(),
        "--manifest-path".into(),
        manifest_path.as_ref().into(),
    ];

    Ok({
        cargo_command("update", &update_options, &prjdir).map_err(|e| {
            error!(err = %e);
            OBSCargoError::new(
                OBSCargoErrorKind::VendorError,
                "Unable to execute cargo".to_string(),
            )
        })?;
        info!("⏫ Successfully ran cargo update");
    })
}

pub fn generate_lockfile(manifest_path: impl AsRef<Path>) -> Result<(), OBSCargoError> {
    let lockfile_options: Vec<OsString> = vec![
        "-vv".into(),
        "--manifest-path".into(),
        manifest_path.as_ref().into(),
    ];

    let parent_path = if let Some(the_parent) = manifest_path.as_ref().parent() {
        the_parent.to_path_buf()
    } else {
        let guess_path = std::env::current_dir().map_err(|e| {
            error!(err = %e);
            OBSCargoError::new(
                OBSCargoErrorKind::LockFileError,
                "Getting parent path for lockfile generation failed".into(),
            )
        })?;
        guess_path.to_path_buf()
    };

    cargo_command("generate-lockfile", &lockfile_options, parent_path).map_err(|e| {
        error!(err = %e);
        OBSCargoError::new(
            OBSCargoErrorKind::LockFileError,
            "Unable to generate a lockfile".into(),
        )
    })?;
    info!("🔒 Successfully generated lockfile");
    Ok(())
}

pub fn vendor(
    prjdir: impl AsRef<Path>,
    cargo_config: impl AsRef<Path>,
    manifest_path: impl AsRef<Path>,
    extra_manifest_paths: &[impl AsRef<Path>],
    filter: bool,
    respect_lockfile: bool,
    versioned_dirs: bool,
) -> Result<(), OBSCargoError> {

    let mut vendor_options: Vec<OsString> =
        vec!["--manifest-path".into(), manifest_path.as_ref().into()];

    for ex_path in extra_manifest_paths {
        vendor_options.push("--sync".into());
        vendor_options.push(ex_path.as_ref().into());
    }

    if versioned_dirs {
        vendor_options.push("--versioned-dirs".into());
    }

    let cargo_subcommand = if filter {
        info!("Filter set to true. Only vendoring crates for platforms *-unknown-linux-gnu and wasm32-*");
        vendor_options.push("--platform=*-unknown-linux-gnu".into());
        // Some crates compile their plugins to WASM, so we need those dependencies as well.
        // Conservatively adding them everywhere, even if they are not needed everywhere.
        // But the impact should be small.
        vendor_options.push("--platform=wasm32-wasi".into());
        vendor_options.push("--platform=wasm32-unknown-unknown".into());
        // We are conservative here and vendor all possible features, even
        // if they are not used in the spec. But we can't know.
        // Maybe make this configurable?
        vendor_options.push("--all-features".into());
        // vendor-filterer could theoretically also create the tarballs for us,
        // with using `--format=tar.zstd` for example. But we need to include
        // additional files and it also doesn't support all compression-schemes.
        vendor_options.push("--format=dir".into());
        if respect_lockfile {
            info!("⚠️ Using vendor-filterer, lockfile verification not supported");
        };
        "vendor-filterer"
    } else {
        // cargo-vendor-filterer doesn't support `-vv`
        vendor_options.push("-vv".into());
        if respect_lockfile {
            // NOTE: Only vendor has the --locked option
            vendor_options.push("--locked".into());
        };
        "vendor"
    };

    debug!(?vendor_options);

    let cargo_vendor_output =
        cargo_command(cargo_subcommand, &vendor_options, &prjdir).map_err(|e| {
            error!(err = %e);
            OBSCargoError::new(
                OBSCargoErrorKind::VendorError,
                "Unable to execute cargo".to_string(),
            )
        })?;

    if let Some(p_path) = cargo_config.as_ref().parent() {
        fs::create_dir_all(p_path).map_err(|err| {
            error!(?err, "Failed to create parent dir for cargo config");
            OBSCargoError::new(
                OBSCargoErrorKind::VendorError,
                "failed to create parent dir for cargo config".to_string(),
            )
        })?;
    }

    let mut file_cargo_config = fs::File::create(cargo_config.as_ref()).map_err(|err| {
        error!(?err, "Failed to create file for cargo config");
        OBSCargoError::new(
            OBSCargoErrorKind::VendorError,
            "failed to create cargo config file".to_string(),
        )
    })?;
    // Write the stdout which is used by the package later.
    file_cargo_config
        .write_all(cargo_vendor_output.as_bytes())
        .map_err(|err| {
            error!(?err, "Failed to write to file for cargo config");
            OBSCargoError::new(
                OBSCargoErrorKind::VendorError,
                "failed to write to file for cargo config".to_string(),
            )
        })
}

pub fn is_workspace(src: &Path) -> Result<bool, OBSCargoError> {
    if let Ok(manifest) = fs::read_to_string(src) {
        if let Ok(manifest_data) = toml::from_str::<toml::Value>(&manifest) {
            if manifest_data.get("workspace").is_some() {
                return Ok(true);
            } else {
                return Ok(false);
            };
        };
    }
    Err(OBSCargoError::new(
        OBSCargoErrorKind::VendorError,
        format!(
            "failed to check manifest file at path {}",
            src.to_string_lossy()
        ),
    ))
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct TomlManifest {
    dependencies: Option<BTreeMap<String, toml::Value>>,
    dev_dependencies: Option<BTreeMap<String, toml::Value>>,
    build_dependencies: Option<BTreeMap<String, toml::Value>>,
    target: Option<BTreeMap<String, toml::Value>>,
}

pub fn has_dependencies(src: &Path) -> Result<bool, OBSCargoError> {
    if let Ok(manifest) = fs::read_to_string(src) {
        match toml::from_str::<TomlManifest>(&manifest) {
            Ok(manifest_data) => {
                debug!("Manifest TOML data: {:?}", manifest_data);
                return Ok(match manifest_data.dependencies {
                    Some(deps) => !deps.is_empty(),
                    None => false,
                } || match manifest_data.dev_dependencies {
                    Some(deps) => !deps.is_empty(),
                    None => false,
                } || match manifest_data.build_dependencies {
                    Some(deps) => !deps.is_empty(),
                    None => false,
                } || match manifest_data.target {
                    Some(deps) => !deps.is_empty(),
                    None => false,
                });
            }
            Err(err) => {
                error!("Failed to deserialize TOML manifest file: {}", err);
                return Err(OBSCargoError::new(
                    OBSCargoErrorKind::VendorError,
                    "Failed to deserialize TOML manifest file".to_string(),
                ));
            }
        };
    }
    Err(OBSCargoError::new(
        OBSCargoErrorKind::VendorError,
        format!(
            "failed to check manifest file at path {}",
            src.to_string_lossy()
        ),
    ))
}
