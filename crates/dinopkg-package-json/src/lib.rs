use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

mod util;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageJson {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub main: Option<String>,
    pub scripts: Option<Scripts>,
    pub dependencies: Option<Dependencies>,
    pub dev_dependencies: Option<Dependencies>,
}

pub type Scripts = HashMap<String, String>;
pub type Dependencies = HashMap<String, String>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("deserialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[cfg(feature = "tokio")]
    #[error("package.json not found")]
    NotFound,

    #[cfg(feature = "tokio")]
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[cfg(feature = "tokio")]
    #[error("file is invalid utf-8")]
    Utf8(#[from] std::string::FromUtf8Error),
}

impl PackageJson {
    pub fn parse(json: &str) -> Result<Self, Error> {
        Ok(serde_json::from_str(json)?)
    }

    #[cfg(feature = "tokio")]
    pub async fn from_file(max_attempts: usize) -> Result<(Self, PathBuf), Error> {
        let path = util::find_package_json(max_attempts).await?;
        let Some(path) = path else {
            return Err(Error::NotFound);
        };
        let file = tokio::fs::read(path.clone()).await?;
        let file = String::from_utf8(file)?;
        Ok((Self::parse(&file)?, path))
    }
}
