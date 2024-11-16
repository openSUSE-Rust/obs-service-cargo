use glob::glob;
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

pub fn run_cargo_vendor(
    setup_workdir: &Path,
    custom_root: &Path,
    vendor_opts: &Opts,
) -> io::Result<()> {
    debug!(?vendor_opts);
    info!("📦 Starting Cargo Vendor");
    let tmpdir_for_config = tempfile::Builder::new()
        .prefix(".cargo_config")
        .rand_bytes(12)
        .tempdir()?;
    let cargo_config_workdir = tmpdir_for_config.path();
    let mut custom_path_for_vendor_dir: String = String::new();
    // Let's attempt a clean environment here too.
    let tempdir_for_home_registry_binding = tempfile::Builder::new()
        .prefix(".cargo")
        .rand_bytes(12)
        .tempdir()?;
    let home_registry = &tempdir_for_home_registry_binding.path();
    let home_registry_dot_cargo = &home_registry.join(".cargo");
    std::env::set_var("CARGO_HOME", home_registry_dot_cargo);
    // Cargo vendor stdouts the configuration for config.toml
    let res = {
        if let Some((lockfile, cargo_config_output)) = cargo_vendor(
            custom_root,
            vendor_opts.vendor_specific_args.versioned_dirs,
            vendor_opts.vendor_specific_args.filter,
            &vendor_opts.manifest_path,
            &vendor_opts.i_accept_the_risk,
            vendor_opts.update,
            &vendor_opts.update_crate,
            vendor_opts.respect_lockfile,
        )? {
            let lockfile_parent = lockfile.parent().unwrap_or(setup_workdir);
            let lockfile_parent_stripped = lockfile_parent
                .strip_prefix(setup_workdir)
                .unwrap_or(setup_workdir);
            custom_path_for_vendor_dir.push_str(&lockfile_parent_stripped.to_string_lossy());
            // NOTE: Both lockfile and dot cargo should have the same parent path.
            let path_to_lockfile = &cargo_config_workdir
                .join(lockfile_parent_stripped)
                .join("Cargo.lock");
            let path_to_dot_cargo = &cargo_config_workdir
                .join(lockfile_parent_stripped)
                .join(".cargo");
            fs::create_dir_all(path_to_dot_cargo)?;
            fs::copy(lockfile, path_to_lockfile)?;
            // NOTE maybe in the future, we might need to respect import
            // an existing `cargo.toml` but I doubt that's necessary?
            let path_to_dot_cargo_cargo_config = &path_to_dot_cargo.join("config.toml");
            let mut cargo_config_file = fs::File::create(path_to_dot_cargo_cargo_config)?;
            cargo_config_file.write_all(cargo_config_output.as_bytes())?;
            debug!(?cargo_config_file);
        } else {
            info!("🎉 Project has no dependencies.");
            return Ok(());
        }
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
        let vendor_path = &custom_root.join("vendor");
        if !vendor_path.is_dir() {
            return Err(io::Error::new(io::ErrorKind::NotFound, "No vendor path found! Please file an issue at <https://github.com/openSUSE-Rust/obs-service-cargo/issues>."));
        }
        // Process them here
        let additional_paths = vec![format!(
            "{},{}",
            vendor_path.to_string_lossy(),
            custom_path_for_vendor_dir
        )];
        let roast_args = RoastArgs {
            target: PathBuf::from(&cargo_config_workdir),
            include: None,
            exclude: None,
            additional_paths: Some(additional_paths),
            outfile,
            outdir: Some(vendor_opts.outdir.to_path_buf()),
            preserve_root: false,
            reproducible: true,
            ignore_git: false,
            ignore_hidden: false,
        };
        roast_opts(&roast_args, false)
    };
    res.map(|val| {
        trace!(?val);
        info!("📦 Cargo Vendor finished.");
        info!("🧹 Cleaning up temporary directory...");
        tmpdir_for_config.close()
    })?
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct TomlManifest {
    dependencies: Option<BTreeMap<String, toml::Value>>,
    dev_dependencies: Option<BTreeMap<String, toml::Value>>,
    build_dependencies: Option<BTreeMap<String, toml::Value>>,
    target: Option<BTreeMap<String, toml::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct WorkspaceTable {
    members: Option<Vec<PathBuf>>,
    default_members: Option<Vec<PathBuf>>,
    #[serde(flatten)]
    extra: Option<TomlManifest>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct WorkspaceTomlManifest {
    workspace: WorkspaceTable,
}

pub fn is_workspace(src: &Path) -> io::Result<bool> {
    if let Ok(manifest) = fs::read_to_string(src) {
        if let Ok(manifest_data) = toml::from_str::<toml::Value>(&manifest) {
            Ok(manifest_data.get("workspace").is_some())
        } else {
            Ok(false)
        }
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "Failed to check manifest file at path {}",
                src.to_string_lossy()
            ),
        ))
    }
}

pub fn workspace_has_dependencies(workdir: &Path, src: &Path) -> io::Result<bool> {
    let mut global_has_deps = false;
    let src_parent = src.parent().unwrap_or(workdir);
    if let Ok(manifest) = fs::read_to_string(src) {
        match toml::from_str::<WorkspaceTomlManifest>(&manifest) {
            Ok(manifest_data) => {
                if let Some(extra_data) = manifest_data.workspace.extra {
                    global_has_deps = match extra_data.dependencies {
                        Some(deps) => !deps.is_empty(),
                        None => false,
                    } || match extra_data.dev_dependencies {
                        Some(deps) => !deps.is_empty(),
                        None => false,
                    } || match extra_data.build_dependencies {
                        Some(deps) => !deps.is_empty(),
                        None => false,
                    }
                }
                let mut members_paths: Vec<PathBuf> = Vec::new();
                if let Some(mut members) = manifest_data.workspace.members {
                    members_paths.append(&mut members);
                };
                if let Some(mut members) = manifest_data.workspace.default_members {
                    members_paths.append(&mut members);
                }
                members_paths.sort();
                members_paths.dedup();
                debug!(?members_paths);
                for member in members_paths {
                    let member_path = src_parent.join(member);
                    let mut member_glob_paths: Vec<PathBuf> = glob(&member_path.to_string_lossy())
                        .map_err(|err| {
                            error!(?err);
                            io::Error::new(io::ErrorKind::NotFound, "Glob pattern not found")
                        })?
                        .flatten()
                        .collect();
                    debug!(?member_glob_paths);
                    while let Some(glob_member_path) = member_glob_paths.pop() {
                        debug!(?glob_member_path);
                        let member_path_from_glob = src_parent.join(glob_member_path);
                        if member_path_from_glob.is_dir() {
                            let possible_manifest_at_path =
                                member_path_from_glob.join("Cargo.toml");
                            if possible_manifest_at_path.is_file() {
                                info!(?possible_manifest_at_path, "🐈 Found a membered path.");
                                let is_workspace = is_workspace(&possible_manifest_at_path)?;
                                if is_workspace {
                                    global_has_deps = global_has_deps
                                        || workspace_has_dependencies(
                                            workdir,
                                            &possible_manifest_at_path,
                                        )?;
                                } else {
                                    global_has_deps = global_has_deps
                                        || has_dependencies(&possible_manifest_at_path)?;
                                }
                            } else {
                                let msg = "The member path does not seem to be a file.";
                                error!(?possible_manifest_at_path, msg);
                                return Err(io::Error::new(io::ErrorKind::NotFound, msg));
                            }
                        } else if member_path_from_glob.is_file() {
                            if let Some(filename) = member_path_from_glob.file_name() {
                                let filename = filename.to_string_lossy();
                                if filename == *"Cargo.toml" {
                                    info!(?member_path_from_glob, "🐈 Found a membered path.");
                                    let is_workspace = is_workspace(&member_path_from_glob)?;
                                    if is_workspace {
                                        global_has_deps = global_has_deps
                                            || workspace_has_dependencies(
                                                workdir,
                                                &member_path_from_glob,
                                            )?;
                                    } else {
                                        global_has_deps = global_has_deps
                                            || has_dependencies(&member_path_from_glob)?;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(err) => {
                error!(?err, "Failed to deserialize TOML manifest file.");
                return Err(io::Error::new(io::ErrorKind::InvalidData, err.to_string()));
            }
        };
        return Ok(global_has_deps);
    };

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
                debug!(?manifest_data, "Manifest TOML data");
                return Ok(match manifest_data.dependencies {
                    Some(deps) => !deps.is_empty(),
                    None => false,
                } || match manifest_data.dev_dependencies {
                    Some(deps) => !deps.is_empty(),
                    None => false,
                } || match manifest_data.build_dependencies {
                    Some(deps) => !deps.is_empty(),
                    None => false,
                });
            }
            Err(err) => {
                debug!(?err, "Failed to deserialize toml.");
            }
        };
    };
    Err(io::Error::new(
        io::ErrorKind::Other,
        format!(
            "Failed to check manifest file at path {}",
            src.to_string_lossy()
        ),
    ))
}
