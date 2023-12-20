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
    builder: &mut tar::Builder<T>,
    prjdir: impl AsRef<Path>,
    archive_files: &[impl AsRef<Path>],
) -> io::Result<()> {
    for f in archive_files.iter().map(|p| p.as_ref()) {
        // Each path is relative to prjdir. So we can split the
        // prjdir prefix to get the relative archive path.
        let f_rel_path = match f.strip_prefix(&prjdir) {
            Ok(f_rel) => f_rel,
            Err(err) => {
                error!(
                    ?err,
                    "THIS IS A BUG. Unable to proceed. {} is not within {}.",
                    f.to_string_lossy(),
                    prjdir.as_ref().to_string_lossy()
                );
                return Err(io::Error::new(io::ErrorKind::Other, f.to_string_lossy()));
            }
        };

        if f.exists() {
            if f.is_file() {
                debug!(?f, "Path to is file!");
                let mut addf = fs::File::open(f)?;
                builder.append_file(f_rel_path, &mut addf)?;
                debug!("Added {} to archive", f.to_string_lossy());
            } else if f.is_dir() {
                builder.append_dir_all(f_rel_path, f)?;
                debug!("Added {} to archive", f.to_string_lossy());
            } else {
                error!(
                    "THIS IS A BUG. Unable to proceed. {} is not a file or directory",
                    f.to_string_lossy()
                );
                return Err(io::Error::new(io::ErrorKind::Other, f.to_string_lossy()));
            };
        } else {
            error!(
                "THIS IS A BUG. Unable to proceed. {} does not exist.",
                f.to_string_lossy()
            );
            return Err(io::Error::new(io::ErrorKind::Other, f.to_string_lossy()));
        }
    }

    builder.finish()
}

pub fn targz(
    outpath: impl AsRef<Path>,
    prjdir: impl AsRef<Path>,
    archive_files: &[impl AsRef<Path>],
) -> io::Result<()> {
    use flate2::write::GzEncoder;
    use flate2::Compression;
    let outtar = fs::File::create(outpath.as_ref()).map_err(|err| {
        error!(outpath = ?outpath.as_ref(), "Unable to create outtar");
        err
    })?;
    let encoder = GzEncoder::new(outtar, Compression::default());
    let mut builder = tar::Builder::new(encoder);
    tar_builder(&mut builder, prjdir, archive_files)
}

pub fn tarzst(
    outpath: impl AsRef<Path>,
    prjdir: impl AsRef<Path>,
    archive_files: &[impl AsRef<Path>],
) -> io::Result<()> {
    use zstd::Encoder;
    let outtar = fs::File::create(outpath.as_ref()).map_err(|err| {
        error!(outpath = ?outpath.as_ref(), "Unable to create outtar");
        err
    })?;
    let mut enc_builder = Encoder::new(outtar, 19)?;
    enc_builder.include_checksum(true)?;
    let threads: u32 = std::thread::available_parallelism()?.get() as u32;
    enc_builder.multithread(threads)?;
    let encoder = enc_builder.auto_finish();
    let mut builder = tar::Builder::new(encoder);
    tar_builder(&mut builder, prjdir, archive_files)
}

pub fn tarxz(
    outpath: impl AsRef<Path>,
    prjdir: impl AsRef<Path>,
    archive_files: &[impl AsRef<Path>],
) -> io::Result<()> {
    // Crc32 is simpler/faster and often hardware accelerated.
    use xz2::stream::Check::Crc32;
    use xz2::stream::MtStreamBuilder;
    use xz2::write::XzEncoder;
    let outtar = fs::File::create(outpath.as_ref()).map_err(|err| {
        error!(outpath = ?outpath.as_ref(), "Unable to create outtar");
        err
    })?;
    let threads: u32 = std::thread::available_parallelism()?.get() as u32;
    let enc_builder = MtStreamBuilder::new()
        .preset(6)
        .threads(threads)
        .check(Crc32)
        .encoder()?;
    let encoder = XzEncoder::new_stream(outtar, enc_builder);
    let mut builder = tar::Builder::new(encoder);
    tar_builder(&mut builder, prjdir, archive_files)
}

pub fn tarbz2(
    outpath: impl AsRef<Path>,
    prjdir: impl AsRef<Path>,
    archive_files: &[impl AsRef<Path>],
) -> io::Result<()> {
    use bzip2::write::BzEncoder;
    use bzip2::Compression;
    let outtar = fs::File::create(outpath.as_ref()).map_err(|err| {
        error!(outpath = ?outpath.as_ref(), "Unable to create outtar");
        err
    })?;
    let encoder = BzEncoder::new(outtar, Compression::best());
    let mut builder = tar::Builder::new(encoder);
    tar_builder(&mut builder, prjdir, archive_files)
}
