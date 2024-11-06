use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use sha2::Digest;
use sha2::Sha256;

#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn, Level};

use crate::audit;
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

pub fn cargo_fetch(curdir: &Path, manifest: &str, mut update: bool) -> io::Result<String> {
    info!("⤵️ Running `cargo fetch`...");
    let mut default_options: Vec<String> = vec![];
    let manifest_path = PathBuf::from(&manifest);
    let manifest_path_parent = manifest_path.parent().unwrap_or(curdir);
    let possible_lockfile = manifest_path_parent.join("Cargo.lock");
    if !update {
        if possible_lockfile.is_file() {
            default_options.push("--locked".to_string());
        } else {
            warn!("⚠️ No lockfile present. This might UPDATE your dependencies. Overriding `update` from false to true.");
            update = true;
        }
    }
    if !manifest.is_empty() {
        default_options.push("--manifest-path".to_string());
        default_options.push(manifest.to_string());
    }
    let res = cargo_command("fetch", &default_options, curdir);
    res.inspect(|_| {
            info!("✅ `cargo fetch` finished!");
    }).inspect_err(
        |err|
        {
            if !update {
                debug!(?err);
                error!("🛑 The lockfile needs to be updated. This operation will fail. Please set the setting `--update` to true.");
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
    mut update: bool,
    i_accept_the_risk: &[String],
) -> io::Result<Option<(PathBuf, String)>> {
    let which_subcommand = if filter { "vendor-filterer" } else { "vendor" };
    info!("🏪 Running `cargo {}`...", &which_subcommand);
    let mut has_update_value_changed = false;
    let mut default_options: Vec<String> = vec![];
    if versioned_dirs {
        default_options.push("--versioned-dirs".to_string());
    }
    let mut first_manifest = curdir.join("Cargo.toml");
    let mut lockfiles: Vec<PathBuf> = Vec::new();
    let mut hasher1 = Sha256::default();
    let mut hasher2 = Sha256::default();
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
    let is_workspace = is_workspace(&first_manifest)?;
    let has_deps = has_dependencies(&first_manifest)?;

    if is_workspace {
        info!("ℹ️ This project is a WORKSPACE configuration.");
        if !workspace_has_dependencies(&first_manifest)? {
            warn!("⚠️ The WORKSPACE MANIFEST does not seem to contain workspace dependencies and dev-dependencies. Please check member dependencies.");
        }
    } else if !has_deps {
        info!("😄 This project does not seem to have any dependencies. Check manifest if we have no need to vendor.");
        info!("🙂 If you think this is a BUG 🐞, please open an issue at <https://github.com/openSUSE-Rust/obs-service-cargo/issues>.");
        return Ok(None);
    }

    manifest_paths.iter().try_for_each(|manifest| {
        let extra_full_manifest_path = curdir.join(manifest);
        if extra_full_manifest_path.exists() {
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
        if !filter {
            default_options.push("--locked".to_string());
        } else {
            warn!("⚠️ Vendor filterer does not support lockfile verification. Your dependencies MIGHT get updated.");
            update = true;
            has_update_value_changed = update;
        }

        info!(?possible_lockfile, "🔓 Adding lockfile.");
        lockfiles.push(possible_lockfile.as_path().to_path_buf());
        let bytes = fs::read(&possible_lockfile)?;
        hasher1.update(&bytes);
    } else {
        warn!(
            "⚠️ No lockfile present. This might UPDATE your dependency. Overriding `update` from \
				 false to true."
        );
        update = true;
        has_update_value_changed = update;
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
        cargo_update(curdir, &first_manifest.to_string_lossy())?;
    }
    cargo_fetch(curdir, &first_manifest.to_string_lossy(), update)?;
    cargo_generate_lockfile(curdir, &first_manifest.to_string_lossy(), update)?;

    // NOTE: Vendor filterer's default output format is directory so we
    // don't need to set that ourselves.
    let res = cargo_command(which_subcommand, &default_options, curdir);

    if possible_lockfile.is_file() {
        default_options.push("--locked".to_string());
        info!(?possible_lockfile, "🔓 Adding lockfile.");
        lockfiles.push(possible_lockfile.as_path().to_path_buf());
        let bytes = fs::read(&possible_lockfile)?;
        hasher2.update(&bytes);
    }
    let hash1 = hex::encode(hasher1.finalize());
    let hash2 = hex::encode(hasher2.finalize());
    if hash1 != hash2 {
        debug!(?hash1, ?hash2);
        warn!("⚠️ Lockfile has changed");
        warn!("Previous hash: {}", hash1);
        warn!("New hash: {}", hash2);
        warn!("⚠️ If you wish to respect the lockfile, consider not setting `--update` to true. However, this MIGHT FAIL in some cases.");
        if has_update_value_changed && update {
            let mut msg: String = "⚠️ Update was SET from FALSE to TRUE , hence a NEW LOCKFILE was CREATED since there was NO LOCKFILE prior. Your dependencies MIGHT have updated.".to_string();
            if filter {
                msg.push_str(" This is because `--filter` option is set to true 🧺.");
            }
            warn!(msg);
        }
    } else {
        info!(
            "🔒 Lockfile was not regenerated for `{}`",
            possible_lockfile.display()
        );
        info!("Previous hash: {}", hash1);
        info!("New hash: {}", hash2);
    }

    info!("🛡️🫥 Auditing lockfiles...");
    if let Ok(audit_result) = audit::perform_cargo_audit(&lockfiles, i_accept_the_risk) {
        audit::process_reports(audit_result).map_err(|err| {
            error!(?err);
            io::Error::new(io::ErrorKind::Interrupted, err.to_string())
        })?;
    }
    info!("🛡️🙂 All lockfiles are audited");
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

pub fn cargo_generate_lockfile(
    curdir: &Path,
    manifest: &str,
    mut update: bool,
) -> io::Result<String> {
    info!("🔓 💂 Running `cargo generate-lockfile`...");
    let mut has_update_value_changed = false;
    let mut hasher1 = Sha256::default();
    let mut hasher2 = Sha256::default();
    let mut default_options: Vec<String> = vec![];
    let manifest_path = PathBuf::from(&manifest);
    let manifest_path_parent = manifest_path.parent().unwrap_or(curdir);
    let possible_lockfile = manifest_path_parent.join("Cargo.lock");
    if !update {
        warn!("😥 Disabled update of dependencies. You should enable this for security updates.");
        if possible_lockfile.is_file() {
            default_options.push("--locked".to_string());
            let lockfile_bytes = fs::read(&possible_lockfile)?;
            hasher1.update(&lockfile_bytes);
        } else {
            warn!("⚠️ No lockfile present. This might UPDATE your dependency. Overriding `update` from false to true.");
            update = true;
            has_update_value_changed = true;
        }
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
    let hash1 = hex::encode(hasher1.finalize());
    let hash2 = hex::encode(hasher2.finalize());
    if hash1 != hash2 {
        debug!(?hash1, ?hash2);
        warn!("⚠️ Lockfile has changed");
        warn!("Previous hash: {}", hash1);
        warn!("New hash: {}", hash2);
        warn!("⚠️ If you wish to respect the lockfile, consider not setting `--update` to true. However, this MIGHT FAIL in some cases.");
        if has_update_value_changed && update {
            warn!("⚠️ Update was SET from FALSE to TRUE , hence a NEW LOCKFILE was CREATED since there was NO LOCKFILE prior. Your dependencies MIGHT have updated.");
        }
    } else {
        info!(
            "🔒 Lockfile was not regenerated for `{}`",
            possible_lockfile.display()
        );
        info!("Previous hash: {}", hash1);
        info!("New hash: {}", hash2);
    }
    res.inspect(|_| {
        info!("🔓 💂 `cargo generate-lockfile` finished.");
    })
    .inspect_err(|err| {
        error!(?err);
    })
}

// Do not set `--locked` here. As explained in <https://doc.rust-lang.org/cargo/commands/cargo-update.html#manifest-options>
pub fn cargo_update(curdir: &Path, manifest: &str) -> io::Result<String> {
    info!("⏫ Updating dependencies...");
    let mut default_options = vec![];
    if !manifest.is_empty() {
        default_options.push("--manifest-path".to_string());
        default_options.push(manifest.to_string());
    }
    cargo_command("update", &default_options, curdir)
        .inspect(|_| {
            info!("✅ Updated dependencies.");
        })
        .inspect_err(|err| {
            error!(?err);
        })
}
