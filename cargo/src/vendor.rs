use libroast::common::Compression;
use libroast::operations::cli::RoastArgs;
use libroast::operations::roast::roast_opts;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn, Level};

use crate::cargo_commands::cargo_vendor;
use crate::cli::Opts;

pub fn run_cargo_vendor(setup_workdir: &Path, vendor_opts: &Opts) -> io::Result<()> {
    debug!(?vendor_opts);
    info!("🛖🏃📦 Starting Cargo Vendor Home Registry");
    let tempdir_for_vendor_binding = tempfile::Builder::new()
        .prefix(".cargo")
        .rand_bytes(12)
        .tempdir()?;
    let vendor_workdir_path = &tempdir_for_vendor_binding.path();
    let vendor_path = vendor_workdir_path.join("vendor");
    // Cargo vendor stdouts the configuration for config.toml
    let cargo_config_output = cargo_vendor(
        setup_workdir,
        vendor_opts.vendor_specific_args.filter,
        &vendor_opts.manifest_paths,
        vendor_opts.update,
        &vendor_path,
    )?;
    let path_to_dot_cargo = &vendor_workdir_path.join(".cargo");
    fs::create_dir(path_to_dot_cargo)?;
    let path_to_dot_cargo_cargo_config = &path_to_dot_cargo.join("config.toml");
    let mut cargo_config_file = fs::File::create(path_to_dot_cargo_cargo_config)?;
    cargo_config_file.write_all(cargo_config_output.as_bytes())?;
    debug!(?cargo_config_file);
    let outfile = match &vendor_opts.tag {
        Some(v) => format!("vendor-{}", v),
        None => "vendor".to_string(),
    };
    let mut outfile = PathBuf::from(outfile);
    let extension = match &vendor_opts.compression {
        Compression::Gz => "tar.gz",
        Compression::Xz => "tar.xz",
        Compression::Zst => "tar.zst",
        Compression::Bz2 => "tar.bz",
        Compression::Not => "tar",
    };

    if !outfile.set_extension(extension) {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Unable to set extension",
        ));
    }
    let roast_args = RoastArgs {
        target: vendor_workdir_path.to_path_buf(),
        include: None,
        exclude: None,
        additional_paths: None,
        outfile,
        outdir: Some(vendor_opts.outdir.to_path_buf()),
        preserve_root: false,
        reproducible: true,
        ignore_git: false,
        ignore_hidden: false,
    };
    roast_opts(&roast_args, false)?;
    info!("📦 Cargo Vendor finished.");
    info!("🧹 Cleaning up temporary directory...");
    tempdir_for_vendor_binding.close()?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct TomlManifest {
    dependencies: Option<BTreeMap<String, toml::Value>>,
    dev_dependencies: Option<BTreeMap<String, toml::Value>>,
    build_dependencies: Option<BTreeMap<String, toml::Value>>,
    target: Option<BTreeMap<String, toml::Value>>,
}

pub fn is_workspace(src: &Path) -> io::Result<bool> {
    if let Ok(manifest) = fs::read_to_string(src) {
        if let Ok(manifest_data) = toml::from_str::<toml::Value>(&manifest) {
            if manifest_data.get("workspace").is_some() {
                return Ok(true);
            } else {
                return Ok(false);
            };
        };
    }
    Err(io::Error::new(
        io::ErrorKind::Other,
        format!(
            "Failed to check manifest file at path {}",
            src.to_string_lossy()
        ),
    ))
}

pub fn has_dependencies(src: &Path) -> io::Result<bool> {
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
                return Err(io::Error::new(io::ErrorKind::InvalidData, err.to_string()));
            }
        };
    }
    Err(io::Error::new(
        io::ErrorKind::Other,
        format!(
            "Failed to check manifest file at path {}",
            src.to_string_lossy()
        ),
    ))
}
