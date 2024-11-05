use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use libroast::common::Compression;
use libroast::operations::cli::RoastArgs;
use libroast::operations::roast::roast_opts;
#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn, Level};

use crate::audit;
use crate::cargo_commands::*;
use crate::cli::Opts;

pub fn run_cargo_vendor_home_registry(setup_workdir: &Path, registry: &Opts) -> io::Result<()> {
    debug!(?registry);
    info!("🛖🏃📦 Starting Cargo Vendor Home Registry");
    let tempdir_for_home_registry_binding = tempfile::Builder::new()
        .prefix(".cargo")
        .rand_bytes(12)
        .tempdir()?;
    let home_registry = &tempdir_for_home_registry_binding.path();
    let home_registry_dot_cargo = &home_registry.join(".cargo");
    std::env::set_var("CARGO_HOME", home_registry_dot_cargo);
    debug!(?home_registry_dot_cargo);
    if !registry.no_root_manifest {
        if registry.update {
            cargo_update(setup_workdir, "")?;
        }
        info!(?setup_workdir, "🌳 Finished setting up workdir.");
        info!("🔓Attempting to regenerate lockfile...");
        cargo_generate_lockfile(setup_workdir, "", registry.update)?;
        info!("🔒Regenerated lockfile.");
        info!("🚝 Attempting to fetch dependencies.");
        cargo_fetch(setup_workdir, "", registry.update)?;
        info!("💼 Fetched dependencies.");
    }
    let mut lockfiles: Vec<PathBuf> = Vec::new();
    for manifest in &registry.manifest_paths {
        let full_manifest_path = &setup_workdir.join(manifest);
        let full_manifest_path_parent = full_manifest_path.parent().unwrap_or(setup_workdir);
        if full_manifest_path.is_file() {
            if registry.update {
                info!(
                    ?full_manifest_path,
                    "⏫ Updating dependencies for extra manifest path..."
                );
                cargo_update(
                    full_manifest_path_parent,
                    &full_manifest_path.to_string_lossy(),
                )?;
                info!(
                    ?full_manifest_path,
                    "✅ Updated dependencies for extra manifest path."
                );
            }
            info!(
                ?full_manifest_path,
                "🔓Attempting to regenerate lockfile for extra manifest path..."
            );
            cargo_generate_lockfile(
                full_manifest_path_parent,
                &full_manifest_path.to_string_lossy(),
                registry.update,
            )?;
            info!(
                ?full_manifest_path,
                "🔒Regenerated lockfile for extra manifest path."
            );
            info!(
                ?full_manifest_path,
                "🚝 Attempting to fetch dependencies at extra manifest path..."
            );
            cargo_fetch(
                setup_workdir,
                &full_manifest_path.to_string_lossy(),
                registry.update,
            )?;
            info!(
                ?full_manifest_path,
                "💼 Fetched dependencies for extra manifest path."
            );
        } else {
            let err = io::Error::new(io::ErrorKind::NotFound, "Path to manifest is not a file");
            error!(?err);
            return Err(err);
        }
        let possible_lockfile = full_manifest_path_parent.join("Cargo.lock");
        if possible_lockfile.exists() {
            info!(
                ?possible_lockfile,
                "🔒 👀 Found an extra lockfile. Adding it to home registry for vendoring."
            );
            let stripped_lockfile_path = possible_lockfile
                .strip_prefix(setup_workdir)
                .unwrap_or(&possible_lockfile);
            let new_lockfile_path = &home_registry.join(stripped_lockfile_path);
            let new_lockfile_parent = new_lockfile_path.parent().unwrap_or(home_registry);
            fs::create_dir_all(new_lockfile_parent)?;
            fs::copy(&possible_lockfile, new_lockfile_path)?;
            info!(
                ?possible_lockfile,
                "🔒 🌟 Successfully added extra lockfile."
            );
            lockfiles.push(possible_lockfile.to_path_buf());
        }
    }
    if !registry.no_root_manifest {
        let possible_root_lockfile = &setup_workdir.join("Cargo.lock");
        if possible_root_lockfile.exists() {
            info!(
                ?possible_root_lockfile,
                "🔒 👀 Found the root lockfile. Adding it to home registry for vendoring."
            );
            let stripped_lockfile_path = possible_root_lockfile
                .strip_prefix(setup_workdir)
                .unwrap_or(possible_root_lockfile);
            let new_lockfile_path = &home_registry.join(stripped_lockfile_path);
            let new_lockfile_parent = new_lockfile_path.parent().unwrap_or(home_registry);
            fs::create_dir_all(new_lockfile_parent)?;
            fs::copy(possible_root_lockfile, new_lockfile_path)?;
            info!(
                ?possible_root_lockfile,
                "🔒 🌟 Successfully added the root lockfile."
            );
        }
        lockfiles.push(possible_root_lockfile.to_path_buf());
    }
    info!("🛡️🫥 Auditing lockfiles...");
    if let Ok(audit_result) = audit::perform_cargo_audit(&lockfiles, &registry.i_accept_the_risk) {
        audit::process_reports(audit_result).map_err(|err| {
            error!(?err);
            io::Error::new(io::ErrorKind::Interrupted, err.to_string())
        })?;
    }
    info!("🛡️🙂 All lockfiles are audited");
    info!("👉🏻🗑️ Removing unneeded directories");
    let registry_src_dir = &home_registry_dot_cargo.join("registry").join("src");
    let registry_bin_dir = &home_registry_dot_cargo.join("bin");
    let registry_caches = [".global-cache", ".package-cache", ".package-cache-mutate"];
    if registry_src_dir.exists() {
        info!("🚮 Removing {}", registry_src_dir.display());
        fs::remove_dir_all(registry_src_dir)?;
        info!("🤯 Removed {}", registry_src_dir.display());
    }
    if registry_bin_dir.exists() {
        info!("🚮 Removing {}", registry_bin_dir.display());
        fs::remove_dir_all(registry_bin_dir)?;
        info!("🤯 Removed {}", registry_bin_dir.display());
    }
    for ca in registry_caches {
        let cache = &home_registry_dot_cargo.join(ca);
        if cache.exists() {
            info!("🚮 Removing {}", cache.display());
            fs::remove_file(cache)?;
            info!("🤯 Removed {}", cache.display());
        }
    }
    let outfile = match &registry.tag {
        Some(v) => format!("registry-{}", v),
        None => "registry".to_string(),
    };
    let mut outfile = PathBuf::from(outfile);
    let extension = match &registry.compression {
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
        target: home_registry.to_path_buf(),
        include: None,
        exclude: None,
        additional_paths: None,
        outfile,
        outdir: Some(registry.outdir.to_path_buf()),
        preserve_root: false,
        reproducible: true,
        ignore_git: false,
        ignore_hidden: false,
    };
    roast_opts(&roast_args, false)?;
    info!("📦 Cargo Vendor Home Registry finished.");
    info!("🧹 Cleaning up temporary directory...");
    tempdir_for_home_registry_binding.close()?;
    Ok(())
}
