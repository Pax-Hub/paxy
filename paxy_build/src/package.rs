use lazy_static::lazy_static;
use serde::{Deserialize, Serialize, de::DeserializeOwned, Deserializer};
use snafu::prelude::*;
use std::{fmt::Display, path::PathBuf, str::FromStr};
use url::Url;

#[cfg(feature = "nested_building")]
pub use nested_building::*;

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
    InvalidFileExtension {path: PathBuf,},

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
    ManifestNotFound {
        path: PathBuf,
    },

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

impl<'de> Deserialize<'de> for Author {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        let tokens: Vec<&str> = s.split(['<', '>']).collect(); 
        Ok(Author{
            name: tokens[0].to_owned(),
            email: Some(tokens[1].to_owned()),
        })
    }
}

/// Represents data associated with the package that by itself is not a part
/// of the package
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageMetadata {
    pub name: String,
    pub description: Option<String>,
    pub authors: Option<Vec<Author>>,
    pub author: Option<Author>,
    pub license: Option<String>,
    pub website: Option<Url>,
    pub repository: Option<Url>,
}

/// Represents a generic Uri that could either contain a URL or a path
#[derive(Debug, Serialize, Deserialize)]
pub enum GenericUri {
    Url(Url),
    Path(PathBuf),
}

/// Represents a generic version
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, PartialOrd)]
pub struct Version<V>
where V: PartialOrd {
    version: V,
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

#[cfg(feature = "nested_building")]
mod nested_building {

    use super::*;
    use lazy_static::lazy_static;
    use serde::{Deserialize, Serialize};
    use std::{ffi::OsStr, path::Path, fs::{File, self}, io::Read};
    use walkdir::WalkDir;

    lazy_static! {
        static ref MIN_DEPTH: usize = 1;
        static ref MAX_DEPTH: usize = 5;
    }

    /// Represents one node in a tree of package flavors
    #[derive(Debug, Serialize, Deserialize)]
    pub enum PackageNode<V>
    where V: PartialOrd {
        FlavoredPackage(FlavoredPackage<V>),
        VersionedPackage(VersionedPackage<V>),
    }

    /// Represents a package with possible flavors
    #[derive(Debug, Serialize, Deserialize)]
    pub struct FlavoredPackage<V: PartialOrd> {
        pub flavors: Option<Vec<PackageNode<V>>>,
    }

    /// Represents a leaf package with no flavors, but a version
    #[derive(Debug, Serialize, Deserialize)]
    pub struct VersionedPackage<V>
    where V: PartialOrd {
        #[serde(flatten)]
        pub metadata: PackageMetadata,
        pub version: Option<Version<V>>,
    }

    fn parse_packages<V: PartialOrd + DeserializeOwned + std::fmt::Debug>(package_path: &Path) -> Result<PackageNode<V>, Error> {
        let manifest_paths = WalkDir::new(package_path)
            .follow_links(true) // to respect symlinks
            .min_depth(*MIN_DEPTH) // to at least look inside the directory
            .max_depth(*MAX_DEPTH) // to limit flavor depth to MAX_DEPTH - 1
            .into_iter() // to recursively iterate over the contents of the package, using depth first search (DFS)
            .filter_map(|e| e.ok()) // to eliminate files and directories which cannot be read
            .filter(|e| e.file_type().is_file()) // to eliminate directories
            .map(|e| e.into_path()) // to extract paths
            .filter(|p| p.file_stem() == Some(OsStr::new(*MANIFEST_FILE_STEM))) // to eliminate files which are not named like manifest.[extension]
            .fold(Ok(None), |acc: Result<Option<PackageNode<V>>, Error>, i: PathBuf| -> Result<Option<PackageNode<V>>, Error> { 
                let versioned_package : Result<VersionedPackage<V>, Error>= match i.extension().map(|e| {
                    e.to_str()
                        .map(|es| TryInto::<ManifestFileExtensions>::try_into(es))
                }) {
                    Some(Some(Ok(ManifestFileExtensions::Yaml))) => serde_yaml::from_reader(File::open(&i).context(ManifestCannotBeReadSnafu{
                        path: &i,
                    })?).context(IncorrectYamlManifestFormatSnafu{
                        path: &i,
                    }),
                    Some(Some(Ok(ManifestFileExtensions::Json))) => serde_json::from_reader(File::open(&i).context(ManifestCannotBeReadSnafu{
                        path: &i,
                    })?).context(IncorrectJsonManifestFormatSnafu{
                        path: &i,
                    }),
                    Some(Some(Ok(ManifestFileExtensions::Toml))) => toml::from_slice(fs::read(&i).context(ManifestCannotBeReadSnafu{
                        path: &i,
                    })?.as_slice()).context(IncorrectTomlManifestFormatSnafu{
                        path: &i,
                    }),
                    Some(Some(Ok(ManifestFileExtensions::Ron))) => serde_json::from_reader(File::open(&i).context(ManifestCannotBeReadSnafu{
                        path: &i,
                    })?).context(IncorrectJsonManifestFormatSnafu{
                        path: &i,
                    }),
                    Some(Some(Err(error))) => Err(error)?, // Unsupported extension,
                    Some(None) => Err(Error::InvalidFileExtension { path: i }), // Extension cannot be converted to a string,
                    None => Err(Error::InvalidFileExtension { path: i }), // No extension found,
                };
                println!("Versioned package: {:#?}", versioned_package);
                if let Ok(None) = acc {
                    versioned_package.map(|a| Some(PackageNode::VersionedPackage(a)))
                } else {
                    acc
                }
            });
            todo!();
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        #[cfg(feature = "nested_building")]
        fn parse_nested() {
            let manifest_path = PathBuf::from(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../resources/test/nested_source"
            ));
            assert!(parse_packages::<i32>(&manifest_path).is_ok());
        }
    }
}
