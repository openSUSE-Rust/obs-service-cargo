use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn, Level};

use crate::audit;
use crate::target::TARGET_TRIPLES;
use crate::vendor::has_dependencies;
use crate::vendor::is_workspace;
use crate::vendor::workspace_has_dependencies;

fn cargo_command(
    subcommand: &str,
    options: &[String],
    curdir: impl AsRef<Path>,
) -> io::Result<String> {
    let cmd = std::process::Command::new("cargo")
        .arg(subcommand)
        .args(options.iter())
        .current_dir(curdir.as_ref())
        .output()?;
    trace!(?cmd);
    let stdoutput = String::from_utf8_lossy(&cmd.stdout);
    let stderrput = String::from_utf8_lossy(&cmd.stderr);
    if !cmd.status.success() {
        debug!(?stdoutput);
        debug!(?stderrput);
        return Err(io::Error::new(io::ErrorKind::Interrupted, stderrput));
    };
    debug!(?stdoutput);
    debug!(?stderrput);
    // Return the output on success as this has the infor for .cargo/config
    Ok(stdoutput.to_string())
}

pub fn cargo_fetch(curdir: &Path, manifest: &str, respect_lockfile: bool) -> io::Result<String> {
    info!("⤵️ Running `cargo fetch`...");
    let mut default_options: Vec<String> = vec![];
    let manifest_path = PathBuf::from(&manifest);
    let manifest_path_parent = manifest_path.parent().unwrap_or(curdir);
    let possible_lockfile = manifest_path_parent.join("Cargo.lock");
    if possible_lockfile.is_file() {
        if respect_lockfile {
            default_options.push("--locked".to_string());
        }
    } else {
        info!("🔓Attempting to regenerate lockfile...");
        cargo_generate_lockfile(curdir, manifest)?;
        info!("🔒Regenerated lockfile.");
    }
    if !manifest.is_empty() {
        default_options.push("--manifest-path".to_string());
        default_options.push(manifest.to_string());
    }
    TARGET_TRIPLES.iter().for_each(|target| {
        default_options.push("--target".to_string());
        default_options.push(target.to_string());
    });
    let res = cargo_command("fetch", &default_options, curdir);
    res.inspect(|_| {
            info!("✅ `cargo fetch` finished!");
    }).inspect_err(
        |err|
        {
            if !respect_lockfile {
                debug!(?err);
                error!("🛑 The lockfile needs to be updated. This operation will fail. Please set the setting `--respect-lockfile` to false.");
                error!(?possible_lockfile, "❌ 🔒 Lockfile was not regenerated for and needs update. Aborting gracefully...");
            }
        }
    )
}

pub fn cargo_vendor(
    curdir: &Path,
    versioned_dirs: bool,
    filter: bool,
    manifest_paths: &[PathBuf],
    i_accept_the_risk: &[String],
    update: bool,
    respect_lockfile: bool,
) -> io::Result<Option<(PathBuf, String)>> {
    let which_subcommand = if filter { "vendor-filterer" } else { "vendor" };
    let mut default_options: Vec<String> = vec![];
    if versioned_dirs {
        default_options.push("--versioned-dirs".to_string());
    }
    let mut first_manifest = curdir.join("Cargo.toml");
    let mut lockfiles: Vec<PathBuf> = Vec::new();
    let mut global_has_deps = false;
    if !first_manifest.is_file() {
        warn!("⚠️ Root manifest seems to not exist. Will attempt to fallback to manifest paths.");
        if let Some(first) = &manifest_paths.first() {
            let _first_manifest = &curdir.join(first);
            if _first_manifest.exists() {
                default_options.push("--manifest-path".to_string());
                let string_lossy = &_first_manifest.to_string_lossy();
                default_options.push(string_lossy.to_string());
                first_manifest = _first_manifest.to_path_buf();
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "Failed to vendor as their are no manifest files to use.",
                ));
            }
        } else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Failed to vendor as their are no manifest files to use.",
            ));
        };
    }
    let first_manifest_parent = first_manifest.parent().unwrap_or(curdir);
    let possible_lockfile = first_manifest_parent.join("Cargo.lock");
    let is_manifest_workspace = is_workspace(&first_manifest)?;
    let has_deps = has_dependencies(&first_manifest)?;

    if is_manifest_workspace {
        info!("ℹ️ This manifest is in WORKSPACE configuration.");
        let workspace_has_deps = workspace_has_dependencies(curdir, &first_manifest)?;
        if !workspace_has_deps {
            warn!("⚠️ This WORKSPACE MANIFEST does not seem to contain workspace dependencies and dev-dependencies. Please check member dependencies.");
        }
        global_has_deps = global_has_deps || workspace_has_deps;
    } else if !has_deps {
        info!("😄 This manifest does not seem to have any dependencies.");
        info!("🙂 If you think this is a BUG 🐞, please open an issue at <https://github.com/openSUSE-Rust/obs-service-cargo/issues>.");
    }

    global_has_deps = global_has_deps || has_deps;
    manifest_paths.iter().try_for_each(|manifest| {
        let extra_full_manifest_path = curdir.join(manifest);
        if extra_full_manifest_path.exists() {
            let is_manifest_workspace = is_workspace(&extra_full_manifest_path)?;
            let has_deps = has_dependencies(&extra_full_manifest_path)?;
            if is_manifest_workspace {
                info!("ℹ️ This manifest is in WORKSPACE configuration.");
                let workspace_has_deps = workspace_has_dependencies(curdir, &first_manifest)?;
                if !workspace_has_deps {
                    warn!("⚠️ This WORKSPACE MANIFEST does not seem to contain workspace dependencies and dev-dependencies. Please check member dependencies.");
                }
                global_has_deps = global_has_deps || workspace_has_deps;
            } else if !has_deps {
                info!("😄 This manifest does not seem to have any dependencies.");
                info!("🙂 If you think this is a BUG 🐞, please open an issue at <https://github.com/openSUSE-Rust/obs-service-cargo/issues>.");
            };
            default_options.push("--sync".to_string());
            default_options.push(manifest.to_string_lossy().to_string());
        } else {
            let msg = "Manifest path does not exist. Aborting operation.";
            error!(?extra_full_manifest_path, msg);
            return Err(io::Error::new(io::ErrorKind::NotFound, msg));
        }
        Ok(())
    })?;

    if possible_lockfile.is_file() {
        if filter {
            warn!("⚠️ Vendor filterer does not support lockfile verification. Your dependencies MIGHT get updated.");
        } else if !filter && respect_lockfile {
            default_options.push("--locked".to_string());
        }

        info!(?possible_lockfile, "🔓 Adding lockfile.");
        lockfiles.push(possible_lockfile.as_path().to_path_buf());
    } else {
        warn!(
            "⚠️ No lockfile present. This might UPDATE your dependency. Overriding `update` from \
				 false to true."
        );
        info!("🔓Attempting to regenerate lockfile...");
        cargo_generate_lockfile(curdir, &first_manifest.to_string_lossy())?;
        info!("🔒Regenerated lockfile.");
    }

    if filter {
        default_options.push("--platform=*-unknown-linux-gnu".to_string());
        default_options.push("--platform=wasm32-unknown-unknown".to_string());
        // NOTE: by <https://github.com/msirringhaus>
        // We are conservative here and vendor all possible features, even
        // if they are not used in the spec. But we can't know.
        // Maybe make this configurable?
        // NOTE to that NOTE: by uncomfyhalomacro
        // I think we won't because we can't guess every feature they have.
        // It's usually enabled on `cargo build -F` tbh...
        default_options.push("--all-features".to_string());
    }

    if !update {
        warn!("😥 Disabled update of dependencies. You should enable this for security updates.");
    } else {
        cargo_update(curdir, &first_manifest.to_string_lossy(), respect_lockfile)?;
    }
    info!("🚝 Attempting to fetch dependencies.");
    cargo_fetch(curdir, &first_manifest.to_string_lossy(), respect_lockfile)?;
    info!("💼 Fetched dependencies.");

    // NOTE: Vendor filterer's default output format is directory so we
    // don't need to set that ourselves.
    info!("🏪 Running `cargo {}`...", &which_subcommand);
    let res = cargo_command(which_subcommand, &default_options, curdir);

    if possible_lockfile.is_file() {
        info!(?possible_lockfile, "🔓 Adding lockfile.");
        lockfiles.push(possible_lockfile.as_path().to_path_buf());
    }

    info!("🛡️🫥 Auditing lockfiles...");
    if let Ok(audit_result) = audit::perform_cargo_audit(&lockfiles, i_accept_the_risk) {
        audit::process_reports(audit_result).map_err(|err| {
            error!(?err);
            io::Error::new(io::ErrorKind::Interrupted, err.to_string())
        })?;
    }
    info!("🛡️🙂 All lockfiles are audited");
    if !global_has_deps {
        info!("🎉 Nothing to vendor.");
        return Ok(None);
    }
    match res {
        Ok(output_cargo_configuration) => {
            info!("🏪 `cargo {}` finished.", &which_subcommand);
            Ok(Some((
                possible_lockfile
                    .canonicalize()
                    .unwrap_or(possible_lockfile),
                output_cargo_configuration,
            )))
        }
        Err(err) => {
            error!(?err);
            Err(err)
        }
    }
}

pub fn cargo_generate_lockfile(curdir: &Path, manifest: &str) -> io::Result<String> {
    info!("🔓 💂 Running `cargo generate-lockfile`...");
    let mut hasher = blake3::Hasher::new();
    let mut hasher2 = blake3::Hasher::new();
    let mut default_options: Vec<String> = vec![];
    let manifest_path = PathBuf::from(&manifest);
    let manifest_path_parent = manifest_path.parent().unwrap_or(curdir);
    let possible_lockfile = manifest_path_parent.join("Cargo.lock");
    if possible_lockfile.is_file() {
        let lockfile_bytes = fs::read(&possible_lockfile)?;
        hasher.update(&lockfile_bytes);
    } else {
        warn!("⚠️ No lockfile present. This might UPDATE your dependency. Overriding `update` from false to true.");
    }
    if !manifest.is_empty() {
        default_options.push("--manifest-path".to_string());
        default_options.push(manifest.to_string());
    }
    let res = cargo_command("generate-lockfile", &default_options, curdir);
    if possible_lockfile.exists() {
        let lockfile_bytes = fs::read(&possible_lockfile)?;
        hasher2.update(&lockfile_bytes);
    }
    let hash = hasher.finalize();
    debug!(?hash,);
    warn!("⚠️ New lockfile generated");
    warn!(?hash, "Hash");
    res.inspect(|_| {
        info!("🔓 💂 `cargo generate-lockfile` finished.");
    })
    .inspect_err(|err| {
        error!(?err);
    })
}

pub fn cargo_update(curdir: &Path, manifest: &str, respect_lockfile: bool) -> io::Result<String> {
    info!("⏫ Updating dependencies...");
    let mut default_options = vec![];
    let manifest_path = PathBuf::from(&manifest);
    let manifest_path_parent = manifest_path.parent().unwrap_or(curdir);
    let possible_lockfile = manifest_path_parent.join("Cargo.lock");
    if !manifest.is_empty() {
        default_options.push("--manifest-path".to_string());
        default_options.push(manifest.to_string());
    }
    if possible_lockfile.is_file() && respect_lockfile {
        default_options.push("--locked".to_string());
    }
    cargo_command("update", &default_options, curdir)
        .inspect(|_| {
            info!("✅ Updated dependencies.");
        })
        .inspect_err(|err| {
            error!(?err);
        })
}
