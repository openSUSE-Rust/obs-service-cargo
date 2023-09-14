pub mod compress;
pub mod decompress;

use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn, Level};

pub fn is_workspace(src: &Path) -> Result<bool, io::Error> {
    if let Ok(manifest) = fs::read_to_string(src) {
        if let Ok(manifest_data) = toml::from_str::<toml::Value>(&manifest) {
            if manifest_data.get("workspace").is_some() {
                return Ok(true);
            } else {
                return Ok(false);
            };
        };
    }
    return Err(io::Error::new(
        io::ErrorKind::NotFound,
        src.to_string_lossy(),
    ));
}

pub fn copy_dir_all(src: impl AsRef<Path>, dst: &Path) -> Result<(), io::Error> {
    debug!("Copying sources");
    debug!(?dst);
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        trace!(?entry);
        trace!(?ty);
        if ty.is_dir() {
            trace!(?ty, "Is directory?");
            copy_dir_all(&entry.path(), &dst.join(&entry.file_name()))?;

        // Should we respect symlinks?
        // } else if ty.is_symlink() {
        //     debug!("Is symlink");
        //     let path = fs::read_link(&entry.path())?;
        //     let path = fs::canonicalize(&path).unwrap();
        //     debug!(?path);
        //     let pathfilename = path.file_name().unwrap_or(OsStr::new("."));
        //     if path.is_dir() {
        //         copy_dir_all(&path, &dst.join(pathfilename))?;
        //     } else {
        //         fs::copy(&path, &mut dst.join(pathfilename))?;
        //     }

        // Be pedantic or you get symlink error
        } else if ty.is_file() {
            trace!(?ty, "Is file?");
            fs::copy(&entry.path(), &mut dst.join(&entry.file_name()))?;
        };
    }
    Ok(())
}

// Use path.components() to check length. the least length should be the project root!
pub fn find_file_multiples(
    srcdir: impl AsRef<Path>,
    target_file: &OsStr,
) -> Result<Vec<PathBuf>, io::Error> {
    let mut found: Vec<PathBuf> = Vec::new();
    let rdir = fs::read_dir(&srcdir)?;
    for entry in rdir {
        let entrypath = entry?.path();
        if entrypath != srcdir.as_ref() {
            if entrypath.is_dir() {
                let ancestors = entrypath.ancestors();
                trace!(?ancestors);
                for anc in ancestors {
                    if anc.join(target_file).is_file() && anc.join(target_file).exists() {
                        trace!(?anc, "A file with the filename found");
                        found.push(anc.to_path_buf());
                    } else if anc.as_os_str() == srcdir.as_ref().as_os_str() {
                        trace!(?anc, "Reached root of working directory");
                        if anc.join("Cargo.toml").exists() {
                            found.push(anc.to_path_buf());
                        };
                        break;
                    }
                }
            };
        } else {
            break;
        }
    }
    trace!(?found);
    Ok(found)
}
