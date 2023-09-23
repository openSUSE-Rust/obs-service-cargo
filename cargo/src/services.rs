use std::path::Path;

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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Param {
    #[serde(rename = "@name")]
    pub name: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

impl Services {
    pub fn from_file<P: AsRef<Path>>(p: P) -> std::io::Result<Self> {
        match std::fs::read_to_string(p).map(|content| from_str::<Services>(&content)) {
            Ok(c) => match c {
                Ok(ay) => Ok(ay),
                Err(err) => {
                    error!(?err, "Failed to deserialize xml string");
                    Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Failed to deserialize xml string",
                    ))
                }
            },
            Err(err) => {
                error!(?err, "Failed to read file to string");
                Err(err)
            }
        }
    }
}
