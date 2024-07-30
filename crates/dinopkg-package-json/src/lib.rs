use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none};

mod util;

#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PackageJson {
    pub name: String,
    pub version: String,
    pub author: Option<AuthorVariant>,
    #[serde(default = "default_as_false")]
    #[serde(skip_serializing_if = "is_false")]
    pub private: bool,
    pub license: Option<String>,
    pub description: Option<String>,
    pub main: Option<String>,
    pub repository: Option<RepositoryVariant>,

    pub scripts: Option<Scripts>,

    pub dependencies: Option<Dependencies>,
    pub dev_dependencies: Option<Dependencies>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum AuthorVariant {
    Author { name: String, url: Option<String> },
    String(String),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum RepositoryVariant {
    Repository { r#type: String, url: Option<String> },
    String(String),
}

// serde :/
#[allow(clippy::trivially_copy_pass_by_ref)]
#[inline(always)]
fn is_false(value: &bool) -> bool {
    !value
}

#[inline(always)]
const fn default_as_false() -> bool {
    false
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
