lazy_static! {
    pub static ref CONFIG_FILE_EXTENSIONS: &'static [&'static str] =
        &["toml", "json", "yaml", "yml"];
}

/// Initializes a layered configuration deriving values from the app-wide
/// defaults, overridden by values from global paths, overridden by values from
/// local paths. overridden by environment variables starting with `PAXY_`,
/// overridden by the configuration file specified by the commandline.
/// Values from only files with supported file extensions would be merged.
pub fn init_config<G: GlobalArguments>(
    console_global_arguments: G,
) -> Result<ConfigTemplate, Error> {
    // Initialize configuration with app-wide defaults
    let mut config = Config::new();

    // Update config filepaths in the config data structure. This is one way -
    // The config filepaths are written to the data structure but not retreived
    // from user-provided config files, except for a config file path that is
    // received from the console arguments.
    let config_filepaths = candidate_config_filepaths()?;
    config.figment = config
        .figment
        .admerge(("config_filepaths", &config_filepaths));

    // Merge configuration values from global and local filepaths
    config = config.with_overriding_files(&config_filepaths);

    // Merge configuration values from environment variables
    config = config.with_overriding_env(&format!("{}_", *app::APP_NAME));

    // Merge configuration values from the CLI
    config = config.with_overriding_args(&console_global_arguments);

    // Update log dirpaths in the config data structure. This is one way - The
    // log dirpaths are written to the data structure but not retreived from
    // user-provided config files.
    let preferred_log_dirpath: Option<PathBuf> = config
        .figment
        .extract_inner("log_dirpath")
        .ok();
    let log_dirpath = candidate_log_dirpath(preferred_log_dirpath)?;
    config.figment = config
        .figment
        .admerge(("log_dirpath", &log_dirpath));

    Ok(config.object()?)
}

fn candidate_config_filepaths() -> Result<Vec<PathBuf>, Error> {
    let mut config_filepaths: Vec<PathBuf> = Vec::new();

    // Global directories
    #[cfg(target_family = "unix")]
    config_filepaths.extend(["/etc/xdg".into(), "/etc".into()]);
    #[cfg(target_os = "windows")]
    candidate_config_filepath_stubs.extend([""]);

    // Local directories
    config_filepaths.push(
        directories::BaseDirs::new()
            .context(RetreiveConfigUserAppBaseDirectoriesSnafu {})?
            .config_dir()
            .to_path_buf(),
    );

    // Append filename to create filepath stubs
    config_filepaths
        .iter_mut()
        .for_each(|f| f.push(*app::APP_NAME));

    let lowercase_config_file_extensions = CONFIG_FILE_EXTENSIONS
        .iter()
        .copied()
        .map(str::to_string);
    let uppercase_config_file_extensions = CONFIG_FILE_EXTENSIONS
        .iter()
        .map(|extension| extension.to_uppercase());

    config_filepaths = config_filepaths
        .into_iter()
        .cartesian_product(lowercase_config_file_extensions.chain(uppercase_config_file_extensions))
        .map(|(filepath_stub, extension)| filepath_stub.with_extension(extension))
        .collect();

    Ok(config_filepaths)
}

fn candidate_log_dirpath(preferred_log_dirpath: Option<PathBuf>) -> Result<PathBuf, Error> {
    if let Some(preferred_log_dirpath) = preferred_log_dirpath {
        if !fs::metadata(&preferred_log_dirpath)
            .map(|m| m.permissions())
            .map(|p| p.readonly())
            .unwrap_or(true)
        {
            Ok(preferred_log_dirpath)
        } else {
            Ok(fallback_log_dirpath()?)
        }
    } else {
        Ok(fallback_log_dirpath()?)
    }
}

fn fallback_log_dirpath() -> Result<PathBuf, Error> {
    let xdg_app_dirs =
        directories::BaseDirs::new().context(RetreiveLoggingUserAppBaseDirectoriesSnafu {})?;
    fs::create_dir_all(xdg_app_dirs.data_dir()).context(CreateLogDirectorySnafu {
        path: {
            let mut state_dirpath = xdg_app_dirs
                .data_dir()
                .to_owned();
            state_dirpath.push(*app::APP_NAME);
            state_dirpath
        },
    })?;
    Ok(xdg_app_dirs
        .data_dir()
        .to_owned())
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

// Make `ConfigTemplate` a provider itself for composability.
impl figment::Provider for ConfigTemplate {
    fn metadata(&self) -> figment::Metadata {
        figment::Metadata::named("Config Object")
    }

    fn data(
        &self,
    ) -> Result<figment::value::Map<figment::Profile, figment::value::Dict>, figment::Error> {
        figment::providers::Serialized::defaults(ConfigTemplate::default()).data()
    }

    fn profile(&self) -> Option<figment::Profile> {
        None
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub figment: Figment,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            figment: Figment::from(ConfigTemplate::default()),
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Config::default()
    }

    pub fn with_overriding_file<P: AsRef<Path>>(mut self, filepath: P) -> Self {
        let filepath: &Path = filepath.as_ref();
        if let Some(file_extension) = filepath.extension() {
            let file_extension = file_extension
                .to_string_lossy()
                .to_lowercase();
            match file_extension.as_str() {
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
                &_ => {}
            }
        }

        self
    }

    pub fn with_overriding_files<P, I>(self, filepaths: I) -> Self
    where
        P: AsRef<Path>,
        I: IntoIterator<Item = P>,
    {
        filepaths
            .into_iter()
            .fold(self, |config, filepath| {
                config.with_overriding_file(filepath)
            })
    }

    pub fn with_overriding_filepath_stubs<I1, I2, S, P>(
        mut self,
        file_extensions: I1,
        filepath_stubs: I2,
    ) -> Self
    where
        I1: IntoIterator<Item = S>,
        S: AsRef<str> + Clone,
        I2: IntoIterator<Item = P>,
        <I2 as IntoIterator>::IntoIter: Clone,
        P: Into<PathBuf> + Clone,
    {
        let filepath_stubs = filepath_stubs.into_iter();
        self = file_extensions
            .into_iter()
            .cartesian_product(filepath_stubs)
            .fold(self, |config, (file_extension, filepath_stub)| {
                let mut filepath: PathBuf = filepath_stub.into();
                filepath.set_extension(file_extension.as_ref());

                config.with_overriding_file(filepath)
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
        S: AsRef<str> + Clone,
        P: Into<PathBuf> + Clone,
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
            self.figment = self
                .figment
                .admerge(("config_filepaths", path));
        }

        // Override console output mode from console arguments only if a
        // non-regular output mode is explicitly specified
        let console_output_mode = console_arguments.console_output_mode();
        if console_output_mode != ConsoleOutputMode::Regular {
            self.figment = self
                .figment
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
                self.figment = self
                    .figment
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
            self.figment = self
                .figment
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
    clone::Clone,
    env,
    fs,
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
        display("could not retrieve the XDG base directories for the user"),
        visibility(pub)
    )]
    RetreiveConfigUserAppBaseDirectories {},

    #[non_exhaustive]
    #[snafu(
        display("could not retrieve the XDG base directories for the user"),
        visibility(pub)
    )]
    RetreiveLoggingUserAppBaseDirectories {},

    #[non_exhaustive]
    #[snafu(
        display("could not create the log directory at {:?}: {source}", path),
        visibility(pub)
    )]
    CreateLogDirectory {
        path: PathBuf,
        source: std::io::Error,
    },

    #[non_exhaustive]
    #[snafu(
        display("could not retrieve configuration information: {source}"),
        visibility(pub)
    )]
    ExtractConfig { source: figment::Error },
}

// endregion: ERRORS
