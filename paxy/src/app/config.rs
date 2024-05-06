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

pub struct Config {
    pub figment: Figment,
}

impl Config {
    pub fn new() -> Self {
        Self {
            figment: Figment::from(ConfigTemplate::default()),
        }
    }

    pub fn with_overriding_file<P: AsRef<Path>>(&mut self, filepath: P) -> &mut Self {
        let filepath: &Path = filepath.as_ref();
        if let Some(file_extension) = filepath.extension() {
            file_extension = file_extension
                .to_string_lossy()
                .to_lowercase();
            match (file_extension) {
                "toml" => {
                    self.figment = self
                        .figment
                        .admerge(Toml::file(filepath));
                }
                "json" => {
                    self.figment = self
                        .figment
                        .admerge(Json::file(filepath));
                }
                "yaml" | "yml" => {
                    self.figment = self
                        .figment
                        .admerge(Yaml::file(filepath));
                }
            }
        }

        self
    }

    pub fn with_overriding_files<P, I>(&mut self, filepaths: I) -> &mut Self
    where
        P: AsRef<Path>,
        I: Iterator<Item = P>,
    {
        filepaths.for_each(|filepath| self.with_overriding_file(filepath));

        self
    }

    pub fn with_overriding_env<S: AsRef<str>>(prefix: S) -> &mut Self {
        let prefix = prefix.as_ref();
        self.figment = self
            .figment
            .admerge(Env::prefixed(prefix));

        self
    }

    pub fn with_overriding_env_var<S: AsRef<str>>(env_var_name: S) -> &mut Self {
        let env_var_name = env_var_name.as_ref();
        self.figment = self
            .figment
            .admerge(Env::raw().only(&[env_var_name]));

        self
    }

    pub fn with_overriding_args<A: ui::GlobalArguments>(&mut self, arguments: A) -> &mut Self {
        if let Some(path) = arguments.config_filepath() {
            self.figment = self.figment.admerge(("config_filepaths", path));
        }

        self
    }

    pub fn object(&self) -> Result<ConfigTemplate, Error> {
        self.figment
            .extract()
            .context(ExtractConfigSnafu {})?
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfigTemplate {
    pub config_filepaths: Vec<PathBuf>,
    pub log_filepath_stub: PathBuf,
    pub console_output_format: ConsoleOutputFormat,
}

impl Default for ConfigTemplate {
    fn default() -> Self {
        Self {
            config_filepaths: Vec::new(),
            log_filepath_stub: PathBuf::default(),
            console_output_format: ConsoleOutputFormat::default(),
        }
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

use super::ui::GlobalArguments;
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
