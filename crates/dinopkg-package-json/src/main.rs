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

impl PackageJson {
    pub fn parse(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}
