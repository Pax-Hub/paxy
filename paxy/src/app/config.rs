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
    let mut candidate_config_filepaths: Vec<PathBuf> = Vec::new();

    // Global directories
    #[cfg(target_family = "unix")]
    candidate_config_filepaths.extend(["/etc/xdg".into(), "/etc".into()]);
    #[cfg(target_os = "windows")]
    candidate_config_filepath_stubs.extend([""]);

    // Local directories
    candidate_config_filepaths.push(
        directories::BaseDirs::new()
            .context(RetreiveConfigUserAppBaseDirectoriesSnafu {})?
            .config_dir()
            .to_path_buf(),
    );

    // Append filename to create filepath stubs
    candidate_config_filepaths
        .iter_mut()
        .for_each(|f| f.push(*app::APP_NAME));

    candidate_config_filepaths = candidate_config_filepaths
        .iter()
        .cartesian_product(["toml", "TOML", "json", "JSON", "yaml", "YAML", "yml", "YML"]);

    // Initialize configuration with app-wide defaults
    let mut config = Config::new();

    // Merge configuration values from global and local filepaths
    config = config.with_overriding_files(candidate_config_filepaths);

    // Merge configuration values from environment variables
    config = config.with_overriding_env(&format!("{}_", *app::APP_NAME));

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

    pub fn with_overriding_filepath_stub<P: Into<PathBuf>>(&mut self, filepath_stub: P) -> &mut Self {
        let filepath_stub: PathBuf = filepath_stub.into();
        

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

    pub fn with_overriding_env<S: AsRef<str>>(&mut self, prefix: S) -> &mut Self {
        let prefix = prefix.as_ref();
        self.figment = self
            .figment
            .admerge(Env::prefixed(prefix));

        self
    }

    pub fn with_overriding_args<A: ui::GlobalArguments>(&mut self, cli_arguments: A) -> &mut Self {
        if let Some(path) = cli_arguments.config_filepath() {
            self.figment = self
                .figment
                .admerge(("config_filepaths", path));
        }

        let console_output_mode = cli_arguments.console_output_mode();
        if console_output_mode != ConsoleOutputMode::Regular {
            self.figment = self
                .figment
                .admerge(("console_output_format.mode", console_output_mode));
        }

        let current_max_verbosity = self
            .figment
            .extract_inner::<log::LevelFilter>("console_output_format.max_verbosity");
        let requested_max_verbosity = cli_arguments.max_output_verbosity();
        if let Ok(current_max_verbosity) = current_max_verbosity {
            if cli_requested_max_verbosity > current_max_verbosity {
                self.figment = self
                    .figment
                    .admerge((
                        "console_output_format.max_verbosity",
                        requested_max_verbosity,
                    ))
            }
        }

        let current_no_color = self
            .figment
            .extract_inner::<log::LevelFilter>("console_output_format.no_color");
        let requested_no_color =
            cli_arguments.is_no_color() || cli_arguments.is_plain() || cli_arguments.is_json();
        let env_no_color = env::var("NO_COLOR").is_ok()
            || env::var(format!(
                "{}_NO_COLOR",
                String::from(*app::APP_NAME).to_uppercase()
            ))
            .is_ok()
            || env::var("TERM").is_ok_and(|env_term_value| env_term_value.to_lowercase == "dumb");
        if !current_no_color && (requested_no_color || env_no_color) {
            self.figment = self
                .figment
                .admerge(("console_output_format.no_color", true));
        }

        self
    }

    pub fn object(&self) -> Result<ConfigTemplate, Error> {
        self.figment
            .extract()
            .context(ExtractConfigSnafu {})?
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
use itertools::Itertools;
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use snafu::{OptionExt, ResultExt, Snafu};

use super::ui::{ConsoleOutputMode, GlobalArguments};
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
