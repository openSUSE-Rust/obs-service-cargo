// SPDX-License-Identifier: MPL-2.0

// Copyright (C) 2023  Soc Virnyl Estela

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;
use tar;

#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn};

pub fn tar_builder<T: Write>(
    topdir: &str,
    srcpath: impl AsRef<Path>,
    additional_files: &[impl AsRef<Path>],
    builder: &mut tar::Builder<T>,
) -> Result<(), io::Error> {
    if !additional_files.is_empty() {
        debug!("Adding additional files!");
        for f in additional_files {
            let f_path: &Path = f.as_ref();

            let pathto = &srcpath.as_ref().join(f_path);
            trace!(?pathto);
            let exists = pathto.exists();
            if exists {
                debug!(?pathto, "Path to file or directory exists!");
                if pathto.is_file() {
                    debug!(?pathto, "Path to is file!");
                    let basedir = pathto.file_name().unwrap_or(f_path.as_os_str());
                    let mut addf = fs::File::open(pathto)?;
                    builder.append_file(basedir, &mut addf)?;
                    debug!("Added {} to archive", f_path.to_string_lossy());
                } else if pathto.is_dir() {
                    builder.append_dir_all("", pathto)?;
                    debug!("Added {} to archive", f_path.to_string_lossy());
                } else {
                    warn!(?pathto, "Is this the correct path to file? 🤔");
                };
            };
        }
    };
    builder.append_dir_all(topdir, &srcpath)?;
    builder.finish()?;
    debug!(
        "Successfully created compressed archive for {}",
        srcpath.as_ref().to_string_lossy()
    );
    Ok(())
}

pub fn targz(
    topdir: &str,
    outdir: impl AsRef<Path>,
    srcpath: impl AsRef<Path>,
    additional_files: &[impl AsRef<Path>],
) -> Result<(), io::Error> {
    use flate2::write::GzEncoder;
    use flate2::Compression;
    let outtar = fs::File::create(outdir.as_ref())?;
    let encoder = GzEncoder::new(outtar, Compression::default());
    let mut builder = tar::Builder::new(encoder);
    tar_builder(topdir, srcpath, additional_files, &mut builder)
}

pub fn tarzst(
    topdir: &str,
    outdir: impl AsRef<Path>,
    srcpath: impl AsRef<Path>,
    additional_files: &[impl AsRef<Path>],
) -> Result<(), io::Error> {
    use zstd::Encoder;
    let outtar = fs::File::create(outdir.as_ref())?;
    let mut enc_builder = Encoder::new(outtar, 19)?;
    enc_builder.include_checksum(true)?;
    let threads: u32 = std::thread::available_parallelism()?.get() as u32;
    enc_builder.multithread(threads)?;
    let encoder = enc_builder.auto_finish();
    let mut builder = tar::Builder::new(encoder);
    tar_builder(topdir, srcpath, additional_files, &mut builder)
}

pub fn tarxz(
    topdir: &str,
    outdir: impl AsRef<Path>,
    srcpath: impl AsRef<Path>,
    additional_files: &[impl AsRef<Path>],
) -> Result<(), io::Error> {
    // Crc32 is simpler/faster and often hardware accelerated.
    use xz2::stream::Check::Crc32;
    use xz2::stream::MtStreamBuilder;
    use xz2::write::XzEncoder;
    let outtar = fs::File::create(outdir.as_ref())?;
    let threads: u32 = std::thread::available_parallelism()?.get() as u32;
    let enc_builder = MtStreamBuilder::new()
        .preset(6)
        .threads(threads)
        .check(Crc32)
        .encoder()?;
    let encoder = XzEncoder::new_stream(outtar, enc_builder);
    let mut builder = tar::Builder::new(encoder);
    tar_builder(topdir, srcpath, additional_files, &mut builder)
}
