use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use url::Url;

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    pub repositories: Vec<Url>,
    pub system_install_location: Option<PathBuf>,
    pub user_install_location: Option<PathBuf>,
    pub default_install_type: Option<InstallType>,
}

#[derive(Serialize, Deserialize)]
pub enum InstallType {
    User,
    System,
}
