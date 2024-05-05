/// Initializes a layered configuration deriving values from the app-wide
/// defaults, overridden by values from global paths, overridden by values from
/// local paths. overridden by environment variables starting with `PAXY_`,
/// overridden by the configuration file specified by the commandline.
/// Values from only files with supported file extensions would be merged.
pub fn init_config<G>(cli_global_arguments: G) -> Result<(Config, Vec<PathBuf>), Error>
where
    G: ui::GlobalArguments,
    <G as ui::GlobalArguments>::L: clap_verbosity_flag::LogLevel,
{
    let mut candidate_config_filepath_stubs: Vec<PathBuf> = Vec::new();

    // Global directories
    #[cfg(target_family = "unix")]
    candidate_config_filepath_stubs.extend(["/etc/xdg".into(), "/etc".into()]);
    #[cfg(target_os = "windows")]
    candidate_config_filepath_stubs.extend([""]);

    // Local directories
    candidate_config_filepath_stubs.push(
        directories::BaseDirs::new()
            .context(RetreiveConfigUserAppBaseDirectoriesSnafu {})?
            .config_dir()
            .to_path_buf(),
    );

    // Append filename to create filepath stubs
    candidate_config_filepath_stubs
        .iter_mut()
        .for_each(|f| f.push(*app::APP_NAME));

    // Initialize configuration with app-wide defaults
    let mut figment = Figment::from(Config::default());

    // Merge configuration values from global and local filepaths
    figment = candidate_config_filepath_stubs
        .iter()
        .fold(figment, move |figment, candidate_config_filepath_stub| {
            admerge_from_stub(candidate_config_filepath_stub, figment)
        });

    // Merge configuration values from environment variables
    figment = figment.admerge(Env::prefixed(&format!("{}_", *app::APP_NAME)));

    // Merge configuration values from additional config filepaths (usually
    // specified through CLI)
    if let Some(additional_config_filepath) = preferred_config_filepath {
        if let Some(parent) = additional_config_filepath.parent() {
            if let Some(stem) = additional_config_filepath.file_stem() {
                let mut stub = PathBuf::from(parent);
                stub.push(stem);
                figment = admerge_from_stub(&stub, figment);
                candidate_config_filepath_stubs.push(stub);
            }
        }
    }

    // Merge configuration values from the CLI
    // These are not set to be optional, so only action-required states are
    // merged with the configuration
    if cli_global_arguments.is_uncolored() {
        figment = figment.admerge(("no_color", true));
    }
    if let Some(log_level_filter) = cli_global_arguments.verbosity_filter() {
        figment = figment.admerge(("log_level_filter", log_level_filter));
    }

    Ok((
        figment
            .extract()
            .context(ExtractConfigSnafu {})?,
        candidate_config_filepath_stubs,
    ))
}

fn admerge_from_stub(candidate_config_filepath_stub: &PathBuf, mut figment: Figment) -> Figment {
    figment = figment.admerge(Toml::file(
        candidate_config_filepath_stub.with_extension("toml"),
    ));
    figment = figment.admerge(Json::file(
        candidate_config_filepath_stub.with_extension("json"),
    ));
    figment = figment.admerge(Yaml::file(
        candidate_config_filepath_stub.with_extension("yml"),
    ));
    figment = figment.admerge(Yaml::file(
        candidate_config_filepath_stub.with_extension("yaml"),
    ));
    figment
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub config_dirpaths: Vec<PathBuf>,

    pub log_dirpath: PathBuf,

    pub cli_output_format: ui::CliOutputFormat,
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            config_dirpaths: None,
            log_dirpath: None,
            log_directory: todo!(),
            cli_output_format: todo!(),
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
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use snafu::{OptionExt, ResultExt, Snafu};

use crate::app;
use crate::app::ui;

// endregion: IMPORTS

// region: ERRORS

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

// endregion: ERRORS
