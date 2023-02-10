use lazy_static::lazy_static;
use semver::Version;
use serde::{de::DeserializeOwned, Deserialize, Deserializer, Serialize};
use snafu::prelude::*;
use std::{fmt::Display, marker, path::PathBuf, str::FromStr};
use url::Url;

#[cfg(feature = "nested_sources")]
pub use nested_sources::*;

lazy_static! {
    static ref MANIFEST_FILE_STEM: &'static str = "manifest";
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("The {path_type} path \"{}\" is invalid", path.to_string_lossy()))]
    InvalidPath { path_type: String, path: PathBuf },

    #[snafu(display("Unable to read the manifest file at path \"{}\"", path.to_string_lossy()))]
    ManifestCannotBeRead {
        source: std::io::Error,
        path: PathBuf,
    },

    #[snafu(display(
        "Invalid file extension for manifest_path \"{}\"",
        path.to_string_lossy(),
    ))]
    InvalidFileExtension { path: PathBuf },

    #[snafu(display(
        "Unsupported extension \"{value}\" for \"{value_type}\". Valid extensions are: {:?}",
        valid_values
    ))]
    UnsupportedFileExtension {
        value: String,
        value_type: String,
        valid_values: Vec<String>,
    },

    #[snafu(display(
        "No manifest files found in the path \"{}\"", path.to_string_lossy()
    ))]
    ManifestNotFound { path: PathBuf },

    #[snafu(display(
        "The manifest file at path \"{}\" has incorrect YAML format and hence cannot be parsed", path.to_string_lossy()
    ))]
    IncorrectYamlManifestFormat {
        source: serde_yaml::Error,
        path: PathBuf,
    },

    #[snafu(display(
        "The manifest file at path \"{}\" has incorrect JSON format and hence cannot be parsed", path.to_string_lossy()
    ))]
    IncorrectJsonManifestFormat {
        source: serde_json::Error,
        path: PathBuf,
    },

    #[snafu(display(
        "The manifest file at path \"{}\" has incorrect TOML format and hence cannot be parsed", path.to_string_lossy()
    ))]
    IncorrectTomlManifestFormat {
        source: toml::de::Error,
        path: PathBuf,
    },
}

/// Represents data describing a single author
#[derive(Debug, Serialize)]
pub struct Author {
    pub name: String,
    pub email: Option<String>,
}

impl<'de, 'input> Deserialize<'de> for Author
where
    'de: 'input,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        let tokens: Vec<&str> = s.split(['<', '>']).collect();
        Ok(Author {
            name: tokens[0].to_owned(),
            email: Some(tokens[1].to_owned()),
        })
    }
}

/// Represents data associated with the package that by itself is not a part
/// of the package
#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    pub name: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
    pub authors: Option<Vec<Author>>,
    pub author: Option<Author>,
    pub license: Option<String>,
    pub website: Option<Url>,
    pub repository: Option<Url>,
}

/// Represents all the file extensions a manifest file can have
pub enum ManifestFileExtensions {
    Yaml,
    Json,
    Toml,
    Ron,
}

impl<'a> From<&'a ManifestFileExtensions> for &'a str {
    fn from(value: &'a ManifestFileExtensions) -> Self {
        match value {
            ManifestFileExtensions::Yaml => "yaml",
            ManifestFileExtensions::Json => "json",
            ManifestFileExtensions::Toml => "toml",
            ManifestFileExtensions::Ron => "ron",
        }
    }
}

impl Display for ManifestFileExtensions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <&ManifestFileExtensions as Into<&str>>::into(self))
    }
}

impl TryFrom<&str> for ManifestFileExtensions {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "yaml" | "yml" => Ok(ManifestFileExtensions::Yaml),
            "json" => Ok(ManifestFileExtensions::Json),
            "toml" => Ok(ManifestFileExtensions::Toml),
            "ron" => Ok(ManifestFileExtensions::Ron),
            _ => Err(Error::UnsupportedFileExtension {
                value: String::from(value),
                value_type: String::from("manifest file extension"),
                valid_values: vec![
                    String::from("yaml"),
                    String::from("yml"),
                    String::from("json"),
                    String::from("toml"),
                    String::from("ron"),
                ],
            }),
        }
    }
}

impl FromStr for ManifestFileExtensions {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

/// Represents a generic Uri that could either contain a URL or a path
#[derive(Debug, Serialize, Deserialize)]
pub enum GenericUri {
    Url(Url),
    Path(PathBuf),
}

#[cfg(feature = "nested_sources")]
mod nested_sources {

    use super::*;
    use lazy_static::lazy_static;
    use serde::{Deserialize, Serialize};
    use std::{
        ffi::OsStr,
        fs::{self, File},
        path::Path,
    };
    use walkdir::WalkDir;

    lazy_static! {
        static ref MIN_DEPTH: usize = 1;
        static ref MAX_DEPTH: usize = 5;
    }

    fn parse_packages(package_path: &Path) -> Result<Vec<Package>, Error> {
        let packages = WalkDir::new(package_path)
            .follow_links(true) // to respect symlinks
            .min_depth(*MIN_DEPTH) // to at least look inside the directory
            .max_depth(*MAX_DEPTH) // to limit flavor depth to MAX_DEPTH - 1
            .into_iter() // to recursively iterate over the contents of the package, using depth first search (DFS)
            .filter_map(|e| e.ok()) // to eliminate files and directories which cannot be read
            .filter(|e| e.file_type().is_file()) // to eliminate directories
            .map(|e| e.into_path()) // to extract paths
            .filter(|p| p.file_stem() == Some(OsStr::new(*MANIFEST_FILE_STEM))) // to eliminate files which are not named like manifest.[extension]
            .fold(
                Ok(None),
                |mut acc: Result<Option<Vec<Package>>, Error>,
                 manifest_path: PathBuf|
                 -> Result<Option<Vec<Package>>, Error> {
                    let package: Result<Package, Error> = match manifest_path.extension().map(|e| {
                        e.to_str()
                            .map(|es| TryInto::<ManifestFileExtensions>::try_into(es))
                    }) {
                        Some(Some(Ok(ManifestFileExtensions::Yaml))) => serde_yaml::from_reader(
                            File::open(&manifest_path).context(ManifestCannotBeReadSnafu {
                                path: &manifest_path,
                            })?,
                        )
                        .context(IncorrectYamlManifestFormatSnafu {
                            path: &manifest_path,
                        }),
                        Some(Some(Ok(ManifestFileExtensions::Json))) => serde_json::from_reader(
                            File::open(&manifest_path).context(ManifestCannotBeReadSnafu {
                                path: &manifest_path,
                            })?,
                        )
                        .context(IncorrectJsonManifestFormatSnafu {
                            path: &manifest_path,
                        }),
                        Some(Some(Ok(ManifestFileExtensions::Toml))) => toml::from_str(
                            fs::read_to_string(&manifest_path)
                                .context(ManifestCannotBeReadSnafu {
                                    path: &manifest_path,
                                })?
                                .as_str(),
                        )
                        .context(IncorrectTomlManifestFormatSnafu {
                            path: &manifest_path,
                        }),
                        Some(Some(Ok(ManifestFileExtensions::Ron))) => serde_json::from_reader(
                            File::open(&manifest_path).context(ManifestCannotBeReadSnafu {
                                path: &manifest_path,
                            })?,
                        )
                        .context(IncorrectJsonManifestFormatSnafu {
                            path: &manifest_path,
                        }),
                        Some(Some(Err(error))) => Err(error)?, // Unsupported extension,
                        Some(None) => Err(Error::InvalidFileExtension {
                            path: manifest_path,
                        }), // Extension cannot be converted to a string,
                        None => Err(Error::InvalidFileExtension {
                            path: manifest_path,
                        }), // No extension found,
                    };
                    println!("Package: {:#?}", package);
                    if let Ok(package) = package {
                        match &mut acc {
                            Ok(Some(list_of_packages)) => {
                                list_of_packages.push(package);
                            }
                            Ok(None) => {
                                acc = Ok(Some(vec![package]));
                            }
                            _ => {}
                        };
                    }
                    acc
                },
            );
        packages.map(|item| item.unwrap())
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        #[cfg(feature = "nested_sources")]
        fn parse_nested() {
            let manifest_path = PathBuf::from(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../resources/test/nested_source"
            ));
            assert!(parse_packages(&manifest_path).is_ok());
        }
    }
}
