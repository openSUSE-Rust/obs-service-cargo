// SPDX-License-Identifier: MPL-2.0

// Copyright (C) 2023  Soc Virnyl Estela

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::services::{Service, Services};
use obs_service_cargo::cli::{Compression, Opts, Src, Vendor};
use std::path::{Path, PathBuf};
use std::process;
use std::process::Output;
use std::{fs, io};

fn do_services(package_path: &Path) -> io::Result<()> {
    let cwd = std::env::current_dir()?;
    let cwd_plus_package_path_binding = cwd.join(package_path);
    let cwd_plus_package_path_string = cwd_plus_package_path_binding.as_os_str().to_string_lossy();
    let bindmount_option = format!(
        "{}:{}",
        cwd.as_os_str().to_string_lossy(),
        cwd.as_os_str().to_string_lossy()
    );
    let cmd_arguments = vec![
        "--really_quiet",
        "--config",
        "scan.cfg",
        "--cwd",
        &cwd_plus_package_path_string,
        "--bindmount",
        &bindmount_option,
        "/usr/bin/osc",
        "service",
        "ra",
    ];

    let full_command = process::Command::new("nsjail")
        .args(cmd_arguments)
        .current_dir(&cwd)
        .output()
        .map_err(|err| {
            tracing::error!(err = ?err, "Unable to run nsjail");
            err
        })?;

    if full_command.status.success() {
        let command_output = String::from_utf8_lossy(&full_command.stdout);

        tracing::info!("✅ -- services passed");
        tracing::info!("stdout -- {}", command_output);
    } else {
        let command_output = String::from_utf8_lossy(&full_command.stderr);

        tracing::info!("🚨 -- services failed");
        tracing::info!("stderr -- {}", command_output);

        return Err(io::Error::new(
            io::ErrorKind::Interrupted,
            "Services failed to run",
        ));
    }
    Ok(())
}

pub fn osc_checkout_or_update(package_name: &str, basepath: &Path) -> io::Result<PathBuf> {
    let package_path =
        std::path::PathBuf::from(format!("{}:{}", basepath.to_string_lossy(), package_name));
    tracing::info!(
        "⏫ Checkout or update in progress for {}",
        package_path.to_string_lossy()
    );
    if package_path.exists() {
        tracing::info!("osc revert {}", package_path.to_string_lossy());
        let arguments = vec!["revert", "."];
        let out = osc_command(&package_path, &arguments)?;
        if out.status.success() {
            let command_output = String::from_utf8_lossy(&out.stdout);
            tracing::info!("✅ -- osc revert operation success");

            tracing::info!("stdout -- {}", command_output);
        } else {
            let command_output = String::from_utf8_lossy(&out.stderr);
            tracing::info!("🚨 -- osc revert operation failed");

            tracing::info!("stderr -- {}", command_output);
        };
        let arguments = vec!["clean", "."];
        let out = osc_command(&package_path, &arguments)?;
        if out.status.success() {
            let command_output = String::from_utf8_lossy(&out.stdout);
            tracing::info!("✅ -- osc clean operation success");

            tracing::info!("stdout -- {}", command_output);
        } else {
            let command_output = String::from_utf8_lossy(&out.stderr);
            tracing::info!("🚨 -- osc clean operation failed");

            tracing::info!("stderr -- {}", command_output);
        };
        let arguments = vec!["update", "."];
        let out = osc_command(&package_path, &arguments)?;
        if out.status.success() {
            let command_output = String::from_utf8_lossy(&out.stdout);
            tracing::info!("✅ -- osc update operation success");

            tracing::info!("stdout -- {}", command_output);
        } else {
            let command_output = String::from_utf8_lossy(&out.stderr);
            tracing::info!("🚨 -- osc update operation failed");

            tracing::info!("stderr -- {}", command_output);
        };
    } else {
        let arguments = vec!["bco", "."];
        let out = osc_command(&package_path, &arguments)?;

        if out.status.success() {
            let command_output = String::from_utf8_lossy(&out.stdout);
            tracing::info!("✅ -- osc bco operation success");

            tracing::info!("stdout -- {}", command_output);
        } else {
            let command_output = String::from_utf8_lossy(&out.stderr);
            tracing::info!("🚨 -- osc bco operation failed");

            tracing::info!("stderr -- {}", command_output);
        };
    };
    Ok(package_path)
}

pub fn attempt_osc_operation_with_optional_submit(
    package_path: &Path,
    message: &str,
    yolo: bool,
    findout: bool,
) -> io::Result<PathBuf> {
    tracing::info!(
        "📤 Submitting package in progress at {}",
        package_path.to_string_lossy()
    );
    if yolo {
        let arguments = vec!["vc", "-m", message];
        let out = osc_command(package_path, &arguments)?;

        if out.status.success() {
            let command_output = String::from_utf8_lossy(&out.stdout);
            tracing::info!("✅ -- osc vc operation success");

            tracing::info!("stdout -- {}", command_output);
        } else {
            let command_output = String::from_utf8_lossy(&out.stderr);
            tracing::info!("🚨 -- osc vc operation failed");

            tracing::info!("stderr -- {}", command_output);
        };

        let arguments = vec!["ci", "-m", message];
        let out = osc_command(package_path, &arguments)?;

        if out.status.success() {
            let command_output = String::from_utf8_lossy(&out.stdout);
            tracing::info!("✅ -- osc ci operation success");

            tracing::info!("stdout -- {}", command_output);
        } else {
            let command_output = String::from_utf8_lossy(&out.stderr);
            tracing::info!("🚨 -- osc ci operation failed");

            tracing::info!("stderr -- {}", command_output);
        };

        if findout {
            let arguments = vec!["sr", "-m", message];
            let out = osc_command(package_path, &arguments)?;
            if out.status.success() {
                let command_output = String::from_utf8_lossy(&out.stdout);
                tracing::info!("✅ -- osc sr operation success");

                tracing::info!("stdout -- {}", command_output);
            } else {
                let command_output = String::from_utf8_lossy(&out.stderr);
                tracing::info!("🚨 -- osc sr operation failed");

                tracing::info!("stderr -- {}", command_output);
            };

            tracing::info!("📥 Submitted package at {}", package_path.to_string_lossy());
        } else {
            tracing::info!("🫡 -- You must manually run `osc sr -m {message}`");
        };
    } else {
        tracing::info!("💫 You must manually run the following in {}:\n`osc vc -m {}`\n`osc ci -m {}`\n`osc sr -m {}`", package_path.to_string_lossy(), message, message, message);
    };
    Ok(package_path.to_path_buf())
}

fn osc_command(package_path: &Path, arguments: &[&str]) -> io::Result<Output> {
    process::Command::new("osc")
        .args(arguments)
        .current_dir(package_path)
        .output()
}

fn does_have_cargo_vendor(package_path: &Path) -> io::Result<Service> {
    let service_file = package_path.join("_service");
    if !service_file.exists() {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "service file does not exist",
        ))
    } else {
        let content = fs::read_to_string(&service_file)?;
        match content.parse::<Services>() {
            Ok(xml_services) => {
                if let Some(service) = xml_services.service {
                    for is_cargo_vendor in service.iter() {
                        if let Some(name) = &is_cargo_vendor.name {
                            if name == "cargo_vendor" {
                                return Ok(Service {
                                    name: is_cargo_vendor.name.clone(),
                                    param: is_cargo_vendor.param.clone(),
                                    mode: is_cargo_vendor.mode.clone(),
                                });
                            }
                        }
                    }
                    Err(io::Error::new(
                        io::ErrorKind::NotFound,
                        "cargo vendor service not found",
                    ))
                } else {
                    Err(io::Error::new(
                        io::ErrorKind::NotFound,
                        "cargo vendor service not found",
                    ))
                }
            }
            Err(de_err) => {
                tracing::error!(err = ?de_err);
                Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "cannot parse string into xml construct",
                ))
            }
        }
    }
}

pub fn attempt_cargo_update_before_revendor(
    package_path: &Path,
    colorize: clap::ColorChoice,
) -> io::Result<PathBuf> {
    tracing::info!(
        "🔼 Attempting to update in progress at {}",
        package_path.to_string_lossy()
    );
    let cargo_vendor_params = does_have_cargo_vendor(package_path).map_err(|err| {
        tracing::error!(
            "❌ Error -- {} is not setup for cargo vendor!",
            package_path.to_string_lossy()
        );
        err
    })?;
    tracing::info!(
        ?cargo_vendor_params,
        "✅ Success -- {} is setup for cargo vendor!",
        package_path.to_string_lossy()
    );
    do_services(package_path)?;

    // Now start update then vendor
    // Ignore `update` param declared in service file. We attempt to update hence `attempt_update`
    if let Some(params) = cargo_vendor_params.param {
        let update = true;
        let mut compression: String = String::new();
        let mut cargotomls: Vec<PathBuf> = Vec::new();
        let mut accept_risks: Vec<String> = Vec::new();
        let mut src: String = String::new();
        let mut tag: Option<String> = None;
        let outdir = package_path.to_path_buf();
        for param in params.iter() {
            if let Some(name) = param.name.clone() {
                if name == "compression" {
                    if let Some(text) = &param.text {
                        compression.push_str(text);
                    }
                };
                if ["src", "srctar", "srcdir"].contains(&name.as_str()) {
                    if let Some(text) = &param.text {
                        src.push_str(text);
                    }
                };
                if name == "cargotomls" {
                    if let Some(text) = &param.text {
                        cargotomls.push(PathBuf::from(&text));
                    }
                };
                if name == "tag" {
                    if let Some(text) = &param.text {
                        if !text.is_empty() {
                            tag = Some(String::from(text));
                        }
                    }
                };
                if name == "i-accept-the-risk" {
                    if let Some(text) = &param.text {
                        accept_risks.push(String::from(text));
                    }
                };
            }
        }
        if compression.is_empty() || src.is_empty() {
            tracing::error!("🛑 Required parameters are incomplete or empty!");
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "service file may contain incomplete cargo vendor service parameters",
            ));
        };
        let mut comp_type = Compression::default();
        if compression == "gz" {
            comp_type = Compression::Gz;
        } else if compression == "xz" {
            comp_type = Compression::Xz;
        }
        let srcpath = Src {
            src: PathBuf::from(&src),
        };
        let new_opts = Opts {
            src: srcpath.clone(),
            compression: comp_type,
            tag,
            cargotoml: cargotomls,
            update,
            outdir,
            color: colorize,
            i_accept_the_risk: accept_risks,
        };
        srcpath
            .run_vendor(&new_opts)
            .map_err(|obs_service_cargo_error| {
                tracing::error!(err = ?obs_service_cargo_error, "failed to update and vendor");
                io::Error::new(io::ErrorKind::Interrupted, "failed to update and vendor")
            })?;
    } else {
        tracing::error!(
            ?cargo_vendor_params,
            "❌ Service file may contain incomplete service parameters!"
        );
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "service file may contain incomplete cargo vendor service parameters",
        ));
    };

    tracing::info!("✅ Updated package at {}", package_path.to_string_lossy());
    Ok(package_path.to_path_buf())
}
