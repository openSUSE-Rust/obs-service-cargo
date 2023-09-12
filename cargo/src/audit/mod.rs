// SPDX-License-Identifier: MPL-2.0

// Copyright (C) 2023  Soc Virnyl Estela

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use quick_xml as xml;
use std::io;
use std::path::Path;
use std::path::PathBuf;

#[allow(unused_imports)]
use tracing::{debug, error, info, warn, Level};

#[allow(unused_imports)]
use crate::cli::{Opts, SrcDir, SrcTar};

#[allow(dead_code)]
const EXCLUDE_ITEMS: &[&str] = &[
    // These two are excluded because they are fundamentally
    // silly and can never be fixed.
    // https://rustsec.org/advisories/RUSTSEC-2020-0071.html
    // https://rustsec.org/advisories/RUSTSEC-2020-0159.html
    "RUSTSEC-2020-0071",
    "RUSTSEC-2020-0159",
];

pub enum Src {
    Tar(SrcTar),
    Dir(SrcDir),
}

pub fn read_service(p: &Path) -> Result<Src, io::Error> {
    // use xml::events::BytesStart;
    use xml::events::Event;
    use xml::name::QName;
    use xml::reader::Reader;
    if let Ok(mut reader) = Reader::from_file(p) {
        reader.trim_text(true);
        // let mut skip_buf = Vec::new();
        let mut operations = Vec::new();
        let mut is_target_tag = false;
        let mut is_srctar = false;
        let mut is_srcdir = false;

        loop {
            if let Ok(read_event) = reader.read_event_into(&mut operations) {
                match read_event {
                    Event::Start(element) => {
                        if element.name() == QName(b"service") {
                            info!(?element, "Found target tag");
                            for attr in element.attributes() {
                                if let Ok(attr) = attr {
                                    println!(
                                        "Attribute: {} = {}",
                                        String::from_utf8_lossy(attr.key.local_name().as_ref()),
                                        String::from_utf8_lossy(&attr.value)
                                    );
                                    if attr.key == QName(b"name")
                                        && attr.value == "cargo_vendor".as_bytes()
                                    {
                                        is_target_tag = true;
                                    }
                                } else {
                                    error!(?attr);
                                    return Err(io::Error::new(
                                        io::ErrorKind::InvalidData,
                                        "Attribute is unreadable",
                                    ));
                                };
                            }
                        } else if element.name() == QName(b"param") && is_target_tag {
                            for attr in element.attributes() {
                                if let Ok(attr) = attr {
                                    println!(
                                        "Attribute: {} = {}",
                                        String::from_utf8_lossy(attr.key.local_name().as_ref()),
                                        String::from_utf8_lossy(&attr.value)
                                    );
                                    if attr.key == QName(b"name")
                                        && attr.value == "srctar".as_bytes()
                                    {
                                        is_srctar = true;
                                    } else if attr.key == QName(b"name")
                                        && attr.value == "srcdir".as_bytes()
                                    {
                                        is_srcdir = true;
                                    }
                                } else {
                                    error!(?attr);
                                    return Err(io::Error::new(
                                        io::ErrorKind::InvalidData,
                                        "Attribute is unreadable",
                                    ));
                                };
                            }
                        }
                    }
                    Event::Text(val) => {
                        if is_srctar {
                            let srctar =
                                PathBuf::from(String::from_utf8_lossy(val.as_ref()).as_ref());
                            return Ok(Src::Tar(SrcTar { srctar }));
                        } else if is_srcdir {
                            let srcdir =
                                PathBuf::from(String::from_utf8_lossy(val.as_ref()).as_ref());
                            return Ok(Src::Dir(SrcDir { srcdir }));
                        }
                    }
                    Event::Eof => {
                        return Err(io::Error::new(
                            io::ErrorKind::UnexpectedEof,
                            "Reached end of file?",
                        ))
                    }
                    _ => continue,
                }
            }
        }
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Cannot be converted to reader",
        ))
    }
}
