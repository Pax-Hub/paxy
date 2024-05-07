use std::{
    fs::{self, create_dir_all, File},
    io::Write,
    path::PathBuf,
};

use bson::{doc, Document};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{actions::ensure_file, home};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Config {
    pub repositories: Document,
    pub system_install_location: PathBuf,
    pub user_install_location: PathBuf,
    pub default_install_type: InstallType,
}

impl Default for Config {
    fn default() -> Self {
        let mut user: PathBuf = home!();
        user.push(".paxy");
        user.push("pkgs");
        let system = if cfg!(linux) {
            PathBuf::from("/")
        } else {
            PathBuf::from("")
        };
        let mut repo_file = home!();
        repo_file.push(".paxy");
        repo_file.push("repos.bson");


        ensure_file(&repo_file, |mut f: File| {
            let mut buf = Vec::new();
            doc! {"paxy-pkgs": "https://github.com/Pax-Hub/paxy-pkg-repository.git"}
                .to_writer(&mut buf)
                .unwrap();
            f.write_all(&buf)
                .unwrap();
        });
        let repo_file = File::open(repo_file).unwrap();
        let doc = Document::from_reader(repo_file).unwrap();
        let conf = Config {
            repositories: doc,
            user_install_location: user.clone(), /* Not harmful since the value is dropped in the
                                                  * very next line */
            system_install_location: system,
            default_install_type: InstallType::default(),
        };
        if !user.is_dir() {
            create_dir_all(user.clone()).expect("No permission"); // Not harmful since the value is dropped in the soon
            user.pop();
            user.push("config.toml");
            if user.is_file() {
                fs::remove_file(user.clone()).unwrap();
            }
            if !user.is_file() {
                let mut file = File::create(user).unwrap();
                file.write_all(
                    toml::to_string(&conf)
                        .unwrap()
                        .as_bytes(),
                )
                .expect("Permission error");
            }
        }
        conf
    }
}

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
pub enum InstallType {
    User,
    #[default]
    System,
}

#[allow(dead_code)]
fn load_conf() -> Config {
    let mut conf_path: PathBuf = home!();
    conf_path.push(".paxy");
    conf_path.push("config.toml");
    match fs::read_to_string(&conf_path) {
        Ok(val) => match toml::from_str(&val) {
            Ok(toml) => toml,
            Err(_) => panic!("invalid config file"),
        },
        Err(_) => {
            let val = Config::default();
            let toml = toml::to_string_pretty::<Config>(&val).unwrap();
            let mut file = File::create(conf_path).unwrap();
            file.write_all(toml.as_bytes())
                .unwrap();
            val
        }
    }
}

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use super::*;

    #[test]
    #[serial]
    fn load_conf_test_noexist() {
        let mut repo_file = home!();
        repo_file.push(".paxy");
        repo_file.push("repos.bson");
        if repo_file.is_file() {
            fs::remove_file(repo_file).unwrap();
        }
        assert_eq!(load_conf(), Config::default())
    }
}
