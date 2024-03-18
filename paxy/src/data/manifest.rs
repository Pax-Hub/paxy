use serde::{Deserialize, Serialize};
use url::Url;
use semver as sv;

#[derive(Debug, Serialize, Deserialize)]
struct Manifest {
    version: Vec<Version>,
    author: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Version {
    url: Url,
    number: sv::Version,
}