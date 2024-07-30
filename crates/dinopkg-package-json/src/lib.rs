use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none};

mod util;

#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(untagged)]
pub enum AuthorVariant {
    Author { name: String, url: Option<String> },
    String(String),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(untagged)]
pub enum RepositoryVariant {
    Repository { r#type: String, url: Option<String> },
    String(String),
}

// serde :/
#[allow(clippy::trivially_copy_pass_by_ref)]
#[cfg(not(tarpaullin_include))]
#[inline(always)]
fn is_false(value: &bool) -> bool {
    !value
}

#[inline(always)]
#[cfg(not(tarpaullin_include))]
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

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashmap;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse() {
        let json = r#"{
            "name": "dinopkg-package-json",
            "version": "0.1.0",
            "author": "Skyfall",
            "dependencies": {
                "express": "^4.17.1"
            }
        }"#;
        let package_json = PackageJson::parse(json).unwrap();
        assert_eq!(
            package_json,
            PackageJson {
                name: "dinopkg-package-json".into(),
                version: "0.1.0".into(),
                author: Some(AuthorVariant::String("Skyfall".into())),
                dependencies: Some(hashmap! {
                    "express".into() => "^4.17.1".into(),
                }),
                ..Default::default()
            }
        )
    }

    #[test]
    fn author_variants() {
        let json = r#"{
                "name": "dinopkg-package-json",
                "version": "0.1.0",
                "author": {
                    "name": "Skyfall",
                    "url": "https://skyfall.dev"
                }
            }"#;
        let package_json = PackageJson::parse(json).unwrap();
        assert_eq!(
            package_json,
            PackageJson {
                name: "dinopkg-package-json".into(),
                version: "0.1.0".into(),
                author: Some(AuthorVariant::Author {
                    name: "Skyfall".into(),
                    url: Some("https://skyfall.dev".into())
                }),
                ..Default::default()
            }
        )
    }

    #[test]
    fn repository_variants() {
        let json = r#"{
                "name": "dinopkg-package-json",
                "version": "0.1.0",
                "repository": {
                    "type": "git",
                    "url": "git+https://github.com/SkyfallWasTaken/choco.git"
                }
            }"#;
        let package_json = PackageJson::parse(json).unwrap();
        assert_eq!(
            package_json,
            PackageJson {
                name: "dinopkg-package-json".into(),
                version: "0.1.0".into(),
                repository: Some(RepositoryVariant::Repository {
                    r#type: "git".into(),
                    url: Some("git+https://github.com/SkyfallWasTaken/choco.git".into())
                }),
                ..Default::default()
            }
        )
    }
}
