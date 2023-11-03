// SPDX-License-Identifier: MPL-2.0

// Copyright (C) 2023  Soc Virnyl Estela

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::io;
use std::path::Path;
use std::str::FromStr;

use quick_xml as xml;
use serde::Deserialize;
use serde::Serialize;
#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn, Level};
use xml::de::from_str;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Services {
    #[serde(rename = "service")]
    pub service: Option<Vec<Service>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Service {
    #[serde(rename = "@name")]
    pub name: Option<String>,
    #[serde(rename = "@mode")]
    pub mode: Option<String>,
    #[serde(rename = "param")]
    pub param: Option<Vec<Param>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Param {
    #[serde(rename = "@name")]
    pub name: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

impl FromStr for Services {
    type Err = xml::DeError;
    fn from_str(xml_str: &str) -> Result<Self, xml::DeError> {
        from_str::<Services>(xml_str)
    }
}

impl FromStr for Service {
    type Err = xml::DeError;
    fn from_str(xml_str: &str) -> Result<Self, xml::DeError> {
        from_str::<Service>(xml_str)
    }
}

impl Services {
    #[allow(dead_code)]
    pub fn from_file<P: AsRef<Path>>(p: P) -> io::Result<Self> {
        let content = std::fs::read_to_string(p)?;

        match content.parse::<Services>() {
            Ok(xml_data) => Ok(xml_data),
            Err(xml_err) => {
                tracing::error!(?xml_err, "failed to deserialize string to xml construct");
                Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "failed to deserialize input string into xml construct",
                ))
            }
        }
    }
}

impl Service {
    #[allow(dead_code)]
    pub fn from_file<P: AsRef<Path>>(p: P) -> io::Result<Self> {
        let content = std::fs::read_to_string(p)?;

        match content.parse::<Service>() {
            Ok(xml_data) => Ok(xml_data),
            Err(xml_err) => {
                tracing::error!(?xml_err, "failed to deserialize string to xml construct");
                Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "failed to deserialize input string into xml construct",
                ))
            }
        }
    }
}
