use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize)]
pub struct Author {
    pub username: String,
    pub real_name: Option<String>,
    pub email: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PackageMetadata {
    pub name: String,
    pub description: Option<String>,
    pub authors: Option<Vec<Author>>,
    pub license: Option<String>,
    pub website: Option<Url>,
    pub repository: Option<Url>,
}

#[derive(Serialize, Deserialize)]
pub struct Package {
    #[serde(flatten)]
    pub metadata: PackageMetadata,
    pub flavors: HashMap<String, Url>,
}
impl Package {
    #[allow(dead_code)]
    pub async fn fetch_flavor(&self, name: &str) -> Option<Flavor> {
        if let Some(url) = self.flavors.get(name) {
            match url.scheme() {
                "http" | "https" => {
                    let response = reqwest::get(url.clone())
                        .await
                        .unwrap_or_else(|err| match err.status() {
                            Some(_) => panic!(
                                "Package {}'s flavor {} sent a malformed response: {}",
                                self.metadata.name, name, err
                            ),
                            None => panic!("{}", err),
                        });
                    let text = response.text().await.unwrap();
                    let flavor = serde_yaml::from_str::<Flavor>(&text).unwrap_or_else(|err| {
                        panic!(
                            "Package {}'s flavor {} has a malformed manifest: {}",
                            self.metadata.name, name, err
                        )
                    });
                    Some(flavor)
                }
                "file" => {
                    let text = std::fs::read_to_string(
                        url.as_str().trim().strip_prefix("file://").unwrap(),
                    )
                    .unwrap();
                    let flavor = serde_yaml::from_str::<Flavor>(&text).unwrap_or_else(|err| {
                        panic!(
                            "Package {}'s flavor {} has a malformed manifest: {}",
                            self.metadata.name, name, err
                        )
                    });
                    Some(flavor)
                }
                _ => {
                    panic!(
                        "Unsupported URL scheme {} used in package {}",
                        url.scheme(),
                        self.metadata.name
                    )
                }
            }
        } else {
            None
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Flavor {
    pub name: String,
    pub versions: HashMap<String, Url>,
}
impl Flavor {
    #[allow(dead_code)]
    pub async fn fetch_version(&self, name: &str) -> Option<Version> {
        if let Some(url) = self.versions.get(name) {
            match url.scheme() {
                "http" | "https" => {
                    let response = reqwest::get(url.clone())
                        .await
                        .unwrap_or_else(|err| match err.status() {
                            Some(_) => panic!(
                                "Flavor {}'s version {} sent a malformed response: {}",
                                self.name, name, err
                            ),
                            None => panic!("{}", err),
                        });
                    let text = response.text().await.unwrap();
                    let version = serde_yaml::from_str::<Version>(&text).unwrap_or_else(|err| {
                        panic!(
                            "Flavor {}'s version {} has a malformed manifest: {}",
                            self.name, name, err
                        )
                    });
                    Some(version)
                }
                "file" => {
                    let text = std::fs::read_to_string(
                        url.as_str().trim().strip_prefix("file://").unwrap(),
                    )
                    .unwrap();
                    let version = serde_yaml::from_str::<Version>(&text).unwrap_or_else(|err| {
                        panic!(
                            "Flavor {}'s version {} has a malformed manifest: {}",
                            self.name, name, err
                        )
                    });
                    Some(version)
                }
                _ => {
                    panic!(
                        "Unsupported URL scheme {} used in flavor {}",
                        url.scheme(),
                        self.name
                    )
                }
            }
        } else {
            None
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Version {
    pub name: String,
    pub order: u64,
    pub steps: Vec<BuildStepWrapper>,
    pub dependencies: Option<Vec<Dependency>>,
    pub pre_built: Option<Url>,
}
impl Version {
    #[allow(dead_code)]
    pub async fn fetch_pre_built(&self) -> Option<Vec<u8>> {
        match &self.pre_built {
            Some(url) => match url.scheme() {
                "http" | "https" => {
                    let response = reqwest::get(url.clone())
                        .await
                        .unwrap_or_else(|err| match err.status() {
                            Some(_) => panic!(
                                "Version {}'s pre-built binary sent a malformed response: {}",
                                self.name, err
                            ),
                            None => panic!("{}", err),
                        });
                    let bytes = response.bytes().await.unwrap();
                    Some(bytes.to_vec())
                }
                "file" => {
                    let bytes = std::fs::read(url.path()).unwrap();
                    Some(bytes)
                }
                _ => {
                    panic!(
                        "Unsupported URL scheme {} used in version {}",
                        url.scheme(),
                        self.name
                    )
                }
            },
            None => None,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Dependency {
    pub name: String,
    pub version: Option<String>,
    pub flavor: Option<String>,
}

#[repr(transparent)]
#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct BuildStepWrapper(#[serde(with = "serde_yaml::with::singleton_map")] pub BuildStep);

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildStep {
    Clone { repo: Url },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn package_base() {
        let package: Package = serde_yaml::from_str(include_str!("../base_package.yaml")).unwrap();
        assert_eq!(package.metadata.name, "test");
        assert_eq!(package.flavors.len(), 1);
        let flavor = pollster::block_on(package.fetch_flavor("base_flavor")).unwrap();
        assert_eq!(flavor.name, "base_flavor");
        assert_eq!(flavor.versions.len(), 1);
        let version = pollster::block_on(flavor.fetch_version("base_version")).unwrap();
        assert_eq!(version.name, "base_version");
        assert_eq!(version.order, 0);
        assert_eq!(version.steps.len(), 1);
        assert!(version.dependencies.is_none());
    }
}
