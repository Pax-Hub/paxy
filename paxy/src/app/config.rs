lazy_static! {
    pub static ref CONFIG_FILE_EXTENSIONS: [&'static str] = ["toml", "json", "yaml", "yml"];
}

/// Initializes a layered configuration deriving values from the app-wide
/// defaults, overridden by values from global paths, overridden by values from
/// local paths. overridden by environment variables starting with `PAXY_`,
/// overridden by the configuration file specified by the commandline.
/// Values from only files with supported file extensions would be merged.
pub fn init_config<G>(console_global_arguments: &G) -> Result<(ConfigTemplate, Vec<PathBuf>), Error>
where
    G: ui::GlobalArguments,
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

    let lowercase_config_file_extensions = *CONFIG_FILE_EXTENSIONS.iter();
    let uppercase_config_file_extensions = lowercase_config_file_extensions.map(str::to_uppercase);

    candidate_config_filepaths = candidate_config_filepaths
        .iter()
        .cartesian_product(
            lowercase_config_file_extensions.chain(uppercase_config_file_extensions),
        );

    // Initialize configuration with app-wide defaults
    let mut config = Config::new();

    // Merge configuration values from global and local filepaths
    config = config.with_overriding_files(&candidate_config_filepaths);

    // Merge configuration values from environment variables
    config = config.with_overriding_env(&format!("{}_", *app::APP_NAME));

    // Merge configuration values from the CLI
    config = config.with_overriding_args(console_global_arguments);

    Ok((
        config
            .object()?,
        candidate_config_filepaths,
    ))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfigTemplate {
    pub config_filepaths: Vec<PathBuf>,
    pub log_dirpath: PathBuf,
    pub console_output_format: ui::ConsoleOutputFormat,
}

impl Default for ConfigTemplate {
    fn default() -> Self {
        Self {
            config_filepaths: Vec::new(),
            log_dirpath: PathBuf::default(),
            console_output_format: ui::ConsoleOutputFormat::default(),
        }
    }
}

pub struct Config {
    pub figment: Figment,
}

impl Config {
    pub fn new() -> Self {
        Self {
            figment: Figment::new(),
        }
    }

    pub fn with_overriding_file<P: AsRef<Path>>(mut self, filepath: P) -> Self {
        let filepath: &Path = filepath.as_ref();
        if let Some(file_extension) = filepath.extension() {
            file_extension = file_extension
                .to_string_lossy()
                .to_lowercase();
            match file_extension {
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

    pub fn with_overriding_files<P, I>(self, filepaths: I) -> Self
    where
        P: AsRef<Path>,
        I: IntoIterator<Item = P>,
    {
        filepaths.into_iter().fold(self, |config, filepath| config.with_overriding_file(filepath))
    }

    pub fn with_overriding_filepath_stubs<I1, I2, S, P>(
        mut self,
        file_extensions: I1,
        filepath_stubs: I2,
    ) -> Self
    where
        I1: IntoIterator<Item = S>,
        S: AsRef<str>,
        I2: IntoIterator<Item = P>,
        P: Into<PathBuf>,
    {
        let filepath_stubs = filepath_stubs
            .into_iter()
            .map(Into::into);
        file_extensions
            .into_iter()
            .map(|file_extension| file_extension.as_ref())
            .cartesian_product(&filepath_stubs)
            .map(|(file_extension, filepath_stub)| {
                let filepath = filepath_stub;
                filepath.set_extension(file_extension);

                self = self.with_overriding_file(filepath);
            });

        self
    }

    pub fn with_overriding_filepath_stub<I, S, P>(
        self,
        file_extensions: I,
        filepath_stub: P,
    ) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
        P: Into<PathBuf>,
    {
        self.with_overriding_filepath_stubs(file_extensions, iter::once(filepath_stub))
    }

    pub fn with_overriding_env<S: AsRef<str>>(mut self, prefix: S) -> Self {
        let prefix = prefix.as_ref();
        self.figment = self
            .figment
            .admerge(Env::prefixed(prefix));

        self
    }

    /// Merge configuration values from the CLI
    /// These are not set to be optional, so only action-required states are
    /// merged with the configuration
    pub fn with_overriding_args<A: ui::GlobalArguments>(mut self, console_arguments: A) -> Self {
        // Incorporate the extra config file specified through arguments
        if let Some(path) = console_arguments.config_filepath() {
            self.figment = self.figment
                .admerge(("config_filepaths", path));
        }

        // Override console output mode from console arguments only if a
        // non-regular output mode is explicitly specified
        let console_output_mode = console_arguments.console_output_mode();
        if console_output_mode != ConsoleOutputMode::Regular {
            self.figment = self.figment
                .admerge(("console_output_format.mode", console_output_mode));
        }

        // Override max verbosity from console arguments only if a greater
        // output verbosity is explicitly specified
        let current_max_verbosity = self
            .figment
            .extract_inner::<log::LevelFilter>("console_output_format.max_verbosity");
        let requested_max_verbosity = console_arguments.max_output_verbosity();
        if let Ok(current_max_verbosity) = current_max_verbosity {
            if requested_max_verbosity > current_max_verbosity {
                self.figment = self.figment
                    .admerge((
                        "console_output_format.max_verbosity",
                        requested_max_verbosity,
                    ))
            }
        }

        // Override no-color only if no-color is not already enabled and if
        // either the environment or the console arguments indicate either
        // directly or indirectly that no-color is to be enabled
        let requested_no_color = console_arguments.is_no_color()
            || console_arguments.is_plain()
            || console_arguments.is_json();
        let current_no_color = self
            .figment
            .extract_inner::<bool>("console_output_format.no_color");
        let env_no_color = env::var("NO_COLOR").is_ok()
            || env::var(format!(
                "{}_NO_COLOR",
                String::from(*app::APP_NAME).to_uppercase()
            ))
            .is_ok()
            || env::var("TERM").is_ok_and(|env_term_value| env_term_value.to_lowercase() == "dumb");
        if (requested_no_color || env_no_color) && !current_no_color.unwrap_or(false) {
            self.figment = self.figment
                .admerge(("console_output_format.no_color", true));
        }

        self
    }

    pub fn object(&self) -> Result<ConfigTemplate, Error> {
        let mut config_object: ConfigTemplate = self
            .figment
            .extract()
            .context(ExtractConfigSnafu {})?;

        config_object
            .console_output_format
            .internally_consistent();

        Ok(config_object)
    }
}

// region: IMPORTS

use std::{
    env,
    iter,
    path::{Path, PathBuf},
};

use figment::{
    providers::{Env, Format, Json, Toml, Yaml},
    Figment,
};
use itertools::Itertools;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use snafu::{OptionExt, ResultExt, Snafu};

use super::ui::ConsoleOutputMode;
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
