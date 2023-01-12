use serde::{Deserialize, Serialize};
use snafu::prelude::*;
use std::path::PathBuf;
use url::Url;

#[cfg(feature = "nested_building")]
pub use nested_building::*;

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("The {path_type} path \"{}\" is invalid", path.to_string_lossy()))]
    InvalidPath { path_type: String, path: PathBuf },

    #[snafu(display("Insuficient permissions to access the file/directory at path \"{}\"", path.to_string_lossy()))]
    InsufficientPermissions {
        source: std::io::Error,
        path: PathBuf,
    },
}

/// Represents data describing a single author
#[derive(Debug, Serialize, Deserialize)]
pub struct Author {
    pub username: String,
    pub real_name: Option<String>,
    pub email: Option<Vec<String>>,
}

/// Represents data associated with the package that by itself is not a part
/// of the package
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageMetadata {
    pub name: String,
    pub description: Option<String>,
    pub authors: Option<Vec<Author>>,
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
#[derive(Debug, Serialize, Deserialize)]
pub struct Version<V: PartialOrd> {
    version: V,
}

#[cfg(feature = "nested_building")]
mod nested_building {

    use super::*;
    use serde::{Deserialize, Serialize};
    use snafu::prelude::*;
    use std::{fs, path::Path};

    /// Represents one node in a tree of package flavors
    #[derive(Debug, Serialize, Deserialize)]
    pub enum PackageNode<V: PartialOrd> {
        FlavoredPackage(FlavoredPackage<V>),
        VersionedPackage(VersionedPackage<V>),
    }

    /// Represents a package with possible flavors
    #[derive(Debug, Serialize, Deserialize)]
    pub struct FlavoredPackage<V: PartialOrd> {
        #[serde(flatten)]
        pub metadata: PackageMetadata,
        pub flavors: Option<Vec<PackageNode<V>>>,
    }

    /// Represents a leaf package with no flavors, but a version
    #[derive(Debug, Serialize, Deserialize)]
    pub struct VersionedPackage<V: PartialOrd> {
        #[serde(flatten)]
        pub metadata: PackageMetadata,
        pub version: Version<V>,
    }

    fn parse_packages<V: PartialOrd>(manifest_path: &Path) -> Result<PackageNode<V>, Error> {
        if manifest_path.is_dir() {
            let files: Vec<PathBuf> = manifest_path.read_dir()
                .context(InsufficientPermissionsSnafu {
                    path: manifest_path.to_path_buf(),
                })?
                .filter_map(|r| r.ok()) // Remove entries that result in Err variants and extract data from entries that can be accessed
                .map(|r| r.path()) // Err variants are already removed above, so this is safe
                .collect();
            println!("Files: {:#?}", files);
        }
        todo!()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        #[cfg(feature = "nested_building")]
        fn parse_nested() {
            let manifest_path = PathBuf::from(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../resources/configuration/manifest"
            ));
            assert!(parse_packages::<i32>(&manifest_path).is_ok());
        }
    }
}
