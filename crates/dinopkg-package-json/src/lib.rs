use std::collections::HashMap;

use serde::{Deserialize, Serialize};

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
}

impl PackageJson {
    pub fn parse(json: &str) -> Result<Self, Error> {
        Ok(serde_json::from_str(json)?)
    }

    #[cfg(feature = "tokio")]
    pub async fn from_file() -> Result<Self, Error> {
        let file = tokio::fs::read("package.json").await?;
        let file = String::from_utf8(file)?;
        Self::parse(&file)
    }
}
