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
use crate::vendor::has_dependencies;
use crate::vendor::is_workspace;
use crate::vendor::workspace_has_dependencies;

pub fn run_cargo_vendor_home_registry(
    setup_workdir: &Path,
    custom_root: &Path,
    registry: &Opts,
) -> io::Result<()> {
    debug!(?registry);
    info!("🛖🏃📦 Starting Cargo Vendor Home Registry");
    let tempdir_for_home_registry_binding = tempfile::Builder::new()
        .prefix(".cargo")
        .rand_bytes(12)
        .tempdir()?;
    let home_registry = &tempdir_for_home_registry_binding.path();
    let home_registry_dot_cargo = &home_registry.join(".cargo");
    let mut global_has_deps = false;
    let res = {
        std::env::set_var("CARGO_HOME", home_registry_dot_cargo);
        debug!(?home_registry_dot_cargo);
        if !registry.no_root_manifest {
            let possible_root_manifest = custom_root.join("Cargo.toml");
            if possible_root_manifest.is_file() {
                let is_workspace = is_workspace(&possible_root_manifest)?;
                let has_deps = has_dependencies(&possible_root_manifest)?;
                if is_workspace {
                    info!("ℹ️ This manifest is in WORKSPACE configuration.");
                    let workspace_has_deps = !workspace_has_dependencies(&possible_root_manifest)?;
                    if !workspace_has_deps {
                        warn!("⚠️ This extra WORKSPACE MANIFEST does not seem to contain workspace dependencies and dev-dependencies. Please check member dependencies.");
                    } else {
                        global_has_deps = workspace_has_deps;
                    }
                } else if !has_deps {
                    info!("😄 This extra manifest does not seem to have any dependencies.");
                    info!("🙂 If you think this is a BUG 🐞, please open an issue at <https://github.com/openSUSE-Rust/obs-service-cargo/issues>.");
                    if registry.manifest_path.is_empty() {
                        info!("🎉 No other manifests. No dependencies. Nothing to vendor.");
                        return Ok(());
                    }
                } else {
                    global_has_deps = has_deps;
                }
                if registry.update {
                    cargo_update(custom_root, "")?;
                }
                info!(?setup_workdir, "🌳 Finished setting up workdir.");
                info!("🔓Attempting to regenerate lockfile...");
                cargo_generate_lockfile(custom_root, "", registry.update)?;
                info!("🔒Regenerated lockfile.");
                info!("🚝 Attempting to fetch dependencies.");
                cargo_fetch(custom_root, "", registry.update)?;
                info!("💼 Fetched dependencies.");
            }
        }
        let mut lockfiles: Vec<PathBuf> = Vec::new();
        for manifest in &registry.manifest_path {
            let full_manifest_path = &custom_root.join(manifest);
            let full_manifest_path_parent = full_manifest_path.parent().unwrap_or(setup_workdir);
            if full_manifest_path.is_file() {
                let is_workspace = is_workspace(full_manifest_path)?;
                let has_deps = has_dependencies(full_manifest_path)?;

                if is_workspace {
                    info!("ℹ️ This manifest is in WORKSPACE configuration.");
                    let workspace_has_deps = !workspace_has_dependencies(full_manifest_path)?;
                    if !workspace_has_deps {
                        warn!("⚠️ This extra WORKSPACE MANIFEST does not seem to contain workspace dependencies and dev-dependencies. Please check member dependencies.");
                    } else {
                        global_has_deps = workspace_has_deps;
                    }
                } else if !has_deps {
                    info!("😄 This extra manifest does not seem to have any dependencies.");
                    info!("🙂 If you think this is a BUG 🐞, please open an issue at <https://github.com/openSUSE-Rust/obs-service-cargo/issues>.");
                } else {
                    global_has_deps = has_deps;
                }
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
                    custom_root,
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
            let possible_lockfile = &possible_lockfile
                .canonicalize()
                .unwrap_or(possible_lockfile.to_path_buf());
            if possible_lockfile.exists() {
                info!(
                    ?possible_lockfile,
                    "🔒 👀 Found an extra lockfile. Adding it to home registry for vendoring."
                );
                let stripped_lockfile_path = possible_lockfile
                    .strip_prefix(setup_workdir)
                    .unwrap_or(possible_lockfile);
                let new_lockfile_path = &home_registry.join(stripped_lockfile_path);
                let new_lockfile_parent = new_lockfile_path.parent().unwrap_or(home_registry);
                fs::create_dir_all(new_lockfile_parent)?;
                fs::copy(possible_lockfile, new_lockfile_path)?;
                info!(
                    ?possible_lockfile,
                    "🔒 🌟 Successfully added extra lockfile."
                );
                lockfiles.push(possible_lockfile.to_path_buf());
            }
        }
        if !global_has_deps {
            info!("🎉 Project seems to have no dependencies!");
            info!("🙂 If you think this is a BUG 🐞, please open an issue at <https://github.com/openSUSE-Rust/obs-service-cargo/issues>.");
            return Ok(());
        }
        if !registry.no_root_manifest {
            let possible_root_lockfile = &custom_root.join("Cargo.lock");
            let possible_root_lockfile = &possible_root_lockfile
                .canonicalize()
                .unwrap_or(possible_root_lockfile.to_path_buf());
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
        if let Ok(audit_result) =
            audit::perform_cargo_audit(&lockfiles, &registry.i_accept_the_risk)
        {
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
        Ok(())
    };
    if res.is_ok() {
        info!("📦 Cargo Vendor Home Registry finished.");
        info!("🧹 Cleaning up temporary directory...");
        tempdir_for_home_registry_binding.close()?;
    } else {
        return res;
    }
    Ok(())
}
