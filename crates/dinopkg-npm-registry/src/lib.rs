use std::collections::HashMap;

use dinopkg_package_json::PackageJson;
use serde::{Deserialize, Serialize};

const NPM_REGISTRY_ROOT_URL: &str = "https://registry.npmjs.org";

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageInfo {
    /// The name of the package, for example `discord.js`.
    name: String,

    /// A map of versions to their respective version info.
    ///
    /// The key is the version string (e.g. `0.1.0`), and the value is the version's `package.json` info.
    versions: HashMap<String, PackageJson>,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("network error: {0}")]
    NetworkError(#[from] reqwest::Error),
}

impl PackageInfo {
    pub async fn from_name(
        package_name: &str,
        client: &reqwest::Client,
    ) -> Result<PackageInfo, Error> {
        let url = format!("{NPM_REGISTRY_ROOT_URL}/{package_name}");
        let response = client.get(&url).send().await?;
        let package_info = response.json::<PackageInfo>().await?;
        Ok(package_info)
    }
}
