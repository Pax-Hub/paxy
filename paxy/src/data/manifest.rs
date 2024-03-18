use std::{fs, path::Path};

use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Manifest {
    version: Vec<Version>,
    author: Option<String>,
    name: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Version {
    url: Url,
    number: String,
}

fn parse_manifest(manifest: Box<Path>) -> Manifest {
    let toml = fs::read_to_string(manifest).unwrap();
    toml::from_str(&toml).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_normal() {
        let toml = r#"name = "paxy"
[[version]]
url = "https://github.com/tizonia/tizonia-openmax-il"
number = "1.0.0""#;
        assert_eq!(toml::from_str::<Manifest>(toml).unwrap(), Manifest {
            version: vec![Version {url: Url::parse("https://github.com/tizonia/tizonia-openmax-il").unwrap(), number: "1.0.0".to_string()}],
            name: "paxy".to_string(),
            author: None
        });
    }
}