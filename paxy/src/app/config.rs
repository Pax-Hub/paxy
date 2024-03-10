pub fn init_config() -> Result<(Config, Vec<PathBuf>), Error> {
    let xdg_app_dirs =
        directories::BaseDirs::new().context(RetreiveConfigUserAppBaseDirectoriesSnafu {})?;

    #[cfg(target_os = "linux")]
    let candidate_config_filepath_stubs = vec![
        format!("/etc/xdg/{}", *app::APP_NAME).into(),
        format!("/etc/{}", *app::APP_NAME).into(),
        format!(
            "{}/{}",
            xdg_app_dirs
                .config_dir()
                .to_string_lossy(),
            *app::APP_NAME
        ),
    ];

    let mut figment = Figment::from(Config::default());

    figment = candidate_config_filepath_stubs
        .iter()
        .fold(
            figment,
            move |mut figment, candidate_config_filepath_stub| {
                figment = figment.admerge(Toml::file(format!(
                    "{}.toml",
                    candidate_config_filepath_stub
                )));
                figment = figment.admerge(Json::file(format!(
                    "{}.json",
                    candidate_config_filepath_stub
                )));
                figment = figment.admerge(Yaml::file(format!(
                    "{}.yml",
                    candidate_config_filepath_stub
                )));
                figment = figment.admerge(Yaml::file(format!(
                    "{}.yaml",
                    candidate_config_filepath_stub
                )));
                figment
            },
        );
        
        figment = figment.admerge(Env::prefixed("PAXY_"));

    Ok((
        figment
            .extract()
            .context(ExtractConfigSnafu {})?,
        candidate_config_filepath_stubs
            .iter()
            .map(|f| PathBuf::from(f))
            .collect(),
    ))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub log_directory: Option<String>,

    pub log_level_filter: Option<log::LevelFilter>,

    pub no_color: Option<bool>,
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            log_directory: None,
            log_level_filter: Some(log::LevelFilter::Info),
            no_color: Some(false),
        }
    }
}

impl Provider for Config {
    fn metadata(&self) -> Metadata {
        Metadata::named("Library Config")
    }

    fn data(&self) -> Result<Map<Profile, Dict>, figment::Error> {
        figment::providers::Serialized::defaults(Config::default()).data()
    }

    fn profile(&self) -> Option<Profile> {
        None
    }
}

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(
        display("could not retreive the XDG base directories for the user"),
        visibility(pub)
    )]
    RetreiveConfigUserAppBaseDirectories {},

    #[non_exhaustive]
    #[snafu(
        display("could not retreive configuration information: {source}"),
        visibility(pub)
    )]
    ExtractConfig { source: figment::Error },
}

// region: IMPORTS

use std::path::PathBuf;

use figment::{
    providers::{Env, Format, Json, Toml, Yaml},
    value::{Dict, Map},
    Figment,
    Metadata,
    Profile,
    Provider,
};
use serde::{Deserialize, Serialize};
use snafu::{OptionExt, ResultExt, Snafu};

use crate::app;

// endregion: IMPORTS
