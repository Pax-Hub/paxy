use std::{
    fs::{self, create_dir_all, File},
    io::Write,
    path::PathBuf,
};

use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub repositories: Option<Vec<Url>>,
    pub system_install_location: PathBuf,
    pub user_install_location: PathBuf,
    pub default_install_type: InstallType,
}

impl Default for Config {
    fn default() -> Self {
        let mut user: PathBuf = match home::home_dir() {
            Some(path) => path,
            None => panic!("Impossible to get your home dir!"),
        };
        user.push(".paxy");
        user.push("pkgs");
        let system = if cfg!(unix) {
            PathBuf::from("/")
        } else {
            PathBuf::from("")
        };

        let conf = Config {
            repositories: Some(vec![
                Url::parse("https://github.com/Pax-Hub/Packages.git").unwrap()
            ]),
            user_install_location: user.clone(), /* Not harmful since the value is dropped in the
                                                  * very next line */
            system_install_location: system,
            default_install_type: InstallType::default(),
        };
        if !user.is_dir() {
            create_dir_all(user.clone()).expect("No permission"); // Not harmful since the value is dropped in the soon
            user.pop();
            user.push("config.ini");
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

#[derive(Serialize, Deserialize, Default)]
pub enum InstallType {
    User,
    #[default]
    System,
}

#[allow(dead_code)]
fn load_conf() -> Config {
    let mut conf_path: PathBuf = match home::home_dir() {
        Some(path) => path,
        None => panic!("Impossible to get your home dir!"),
    };
    conf_path.push(".paxy");
    conf_path.push("config.ini");
    match toml::from_str::<Config>(
        fs::read_to_string(&conf_path)
            .unwrap()
            .as_str(),
    ) {
        Ok(val) => val,
        Err(_) => Config::default(),
    }
}
