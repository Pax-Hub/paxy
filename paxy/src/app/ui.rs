//! Objects pertaining to the UI.

pub fn emit_init_messages<C>(config: &ConfigTemplate, console_input: &C)
where
    C: clap::Parser + fmt::Debug,
{
    emit_welcome_messages();
    emit_diagnostic_messages(config);
    emit_test_messages(config, console_input);
}

fn emit_welcome_messages() {
    // Welcome messages
    tracing::debug!(
        "{} - {}",
        "Paxy".bold(),
        "A package manager that gets out of your way".magenta()
    );

    let crate_authors = clap::crate_authors!("\n")
        .split("\n")
        .fold(String::from(""), |mut acc, author| {
            acc.push_str(
                console::Emoji("✉️", "")
                    .to_string()
                    .as_str(),
            );
            acc.push_str("  ");
            acc.push_str(author);
            acc.push('\n');

            acc
        });

    tracing::debug!("{}", crate_authors.italic());
}

fn emit_diagnostic_messages(config: &ConfigTemplate) {
    tracing::trace!(
        "{}  {} messages {}...",
        console::Emoji("🔍", ""),
        "Diagnostic"
            .cyan()
            .dimmed(),
        "begin"
            .green()
            .dimmed(),
    );

    tracing::debug!(
        "{}  The {} is {}... {}",
        console::Emoji("⚙️", ""),
        "configuration".cyan(),
        "loaded".green(),
        console::Emoji("✅", ""),
    );
    tracing::debug!(
        "{} The {} has {}... {}",
        console::Emoji("📝", ""),
        "logging".cyan(),
        "begun".green(),
        console::Emoji("✅", ""),
    );

    let config_filepaths = config
        .config_filepaths
        .iter()
        .fold(String::from(""), |mut acc: String, config_filepath| {
            if config_filepath.is_file() {
                acc.push_str(
                    config_filepath
                        .to_string_lossy()
                        .green()
                        .to_string()
                        .as_str(),
                );
                acc.push_str(", ");
            } else {
                acc.push_str(
                    config_filepath
                        .to_string_lossy()
                        .dimmed()
                        .to_string()
                        .as_str(),
                );
                acc.push_str(
                    ", ".dimmed()
                        .to_string()
                        .as_str(),
                );
            }

            acc
        });

    tracing::debug!(
        "{} {} [{}]",
        console::Emoji("📂", ""),
        "Config Filepath(s):".magenta(),
        config_filepaths,
    );


    tracing::debug!(
        "{} {} {:?}",
        console::Emoji("📂", ""),
        "Log Directory Path:".magenta(),
        config.log_dirpath
    );

    tracing::trace!(
        "{}  {} messages {}...",
        console::Emoji("🔍", ""),
        "Diagnostic"
            .cyan()
            .dimmed(),
        "end"
            .green()
            .dimmed(),
    );
}

fn emit_test_messages<C>(config: &ConfigTemplate, console_input: &C)
where
    C: clap::Parser + fmt::Debug,
{
    tracing::debug!(
        target:"TEST", "{}{}{}{}{}{}{}{}",
        "███".black(),
        "███".red(),
        "███".green(),
        "███".yellow(),
        "███".blue(),
        "███".purple(),
        "███".cyan(),
        "███".white()
    );
    tracing::debug!(
        target:"TEST", "{}{}{}{}{}{}{}{}",
        "███".bright_black(),
        "███".bright_red(),
        "███".bright_green(),
        "███".bright_yellow(),
        "███".bright_blue(),
        "███".bright_purple(),
        "███".bright_cyan(),
        "███".bright_white()
    );

    tracing::trace!(target:"TEST", "{} Testing trace!...", console::Emoji("🧪", ""));
    tracing::debug!(target:"TEST", "{} Testing debug!...", console::Emoji("🧪", ""));
    tracing::info!(target:"TEST", "{} Testing info!...", console::Emoji("🧪", ""));
    tracing::warn!(target:"TEST", "{} Testing warn!...", console::Emoji("🧪", ""));
    tracing::error!(target:"TEST", "{} Testing error!...", console::Emoji("🧪", ""));

    tracing::info!(target:"JSON", "{} Testing: {}", console::Emoji("🧪", ""), "{\"JSON\": \"Target\"}");
    tracing::info!(target:"PLAIN", "{} Testing: Plain Target", console::Emoji("🧪", ""));

    tracing::info!(
        target:"TEST",
        "{}  {} {:#?}",
        console::Emoji("⌨️", ""),
        "CLI input arguments:"
            .magenta()
            .dimmed(),
        console_input.dimmed()
    );

    tracing::info!(
        target:"TEST",
        "{}  {} {:#?}",
        console::Emoji("⌨️", ""),
        "Config dump:"
            .magenta()
            .dimmed(),
        config.dimmed()
    );
}

/// Configurable settings that handle how the console output is displayed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsoleOutputFormat {
    pub mode: ConsoleOutputMode,
    pub max_verbosity: log::LevelFilter,
    pub no_color: bool,
}

impl Default for ConsoleOutputFormat {
    fn default() -> Self {
        Self {
            mode: ConsoleOutputMode::default(),
            max_verbosity: log::LevelFilter::Info,
            no_color: false,
        }
    }
}

impl ConsoleOutputFormat {
    /// Make the console output format internally consistent.
    /// 1. Adjust the no-color setting to be consistent with the console output
    /// mode - there should be no color in plain and json modes.
    /// 2. Adjust the max verbosity to be consistent with the console output
    /// mode - decrease the max verbosity if in plain or json modes.
    pub fn internally_consistent(&mut self) -> &Self {
        self.internally_consistent_color();
        self.internally_consistent_verbosity();

        self
    }

    /// Adjust the no-color setting to be consistent with the console output
    /// mode - there should be no color in plain and json modes.
    /// If color is already disabled, do not enable it. Otherwise toggle
    /// no-color based on the console output mode.
    fn internally_consistent_color(&mut self) -> &Self {
        if !self.no_color
            && matches!(
                self.mode,
                ConsoleOutputMode::Plain | ConsoleOutputMode::Json
            )
        {
            self.no_color = false;
        }

        self
    }

    /// Adjust the max verbosity to be consistent with the console output
    /// mode - decrease the max verbosity if in plain or json modes.
    fn internally_consistent_verbosity(&mut self) -> &Self {
        if self.max_verbosity > log::LevelFilter::Info
            && matches!(
                self.mode,
                ConsoleOutputMode::Plain | ConsoleOutputMode::Json
            )
        {
            self.max_verbosity = log::LevelFilter::Info;
        }

        self
    }
}

/// Represents the output mode of the app. The `Regular` for displaying all
/// logging messages (up to the maximum verbosity set by the user) to the
/// console. The `Plain` mode is for displaying just the main output of the app
/// without any human-facing messages. The `Json` mode also displays the main
/// output of the app, but in the `JSON` format. Both `Plain` and `JSON` modes
/// are to be used to obtain pure scriptable and program-ready outputs without
/// any polluting human messages.
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConsoleOutputMode {
    #[default]
    Regular,
    Plain,
    Json,
    Test,
}

/// A common interface for CLI templates to convey information about standard
/// global arguments  
pub trait GlobalArguments {
    fn config_filepath(&self) -> &Option<PathBuf>;

    fn is_json(&self) -> bool;

    fn is_plain(&self) -> bool;

    fn is_debug(&self) -> bool;

    fn is_no_color(&self) -> bool;

    fn is_test(&self) -> bool;

    fn verbosity_filter(&self) -> log::LevelFilter;

    fn console_output_mode(&self) -> ConsoleOutputMode {
        if self.is_json() {
            ConsoleOutputMode::Json
        } else if self.is_plain() {
            ConsoleOutputMode::Plain
        } else if self.is_test() {
            ConsoleOutputMode::Test
        } else {
            ConsoleOutputMode::Regular
        }
    }

    fn max_output_verbosity(&self) -> log::LevelFilter {
        if self.is_plain() || self.is_json() {
            log::LevelFilter::Info
        } else if self.is_debug() && log::LevelFilter::Debug > self.verbosity_filter() {
            log::LevelFilter::Debug
        } else {
            self.verbosity_filter()
        }
    }
}

/// Blanket implementation of [`GlobalArguments`] interface for *references*
/// of all objects that already implement `GlobalArguments`.
impl<T: GlobalArguments> GlobalArguments for &T {
    fn config_filepath(&self) -> &Option<PathBuf> {
        (**self).config_filepath()
    }

    fn is_json(&self) -> bool {
        (**self).is_json()
    }

    fn is_plain(&self) -> bool {
        (**self).is_plain()
    }

    fn is_debug(&self) -> bool {
        (**self).is_debug()
    }

    fn is_no_color(&self) -> bool {
        (**self).is_no_color()
    }

    fn is_test(&self) -> bool {
        (**self).is_test()
    }

    fn verbosity_filter(&self) -> log::LevelFilter {
        (**self).verbosity_filter()
    }
}

// region: IMPORTS

use std::{fmt, path::PathBuf};

use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

use crate::app::config::ConfigTemplate;

// endregion: IMPORTS

// region: EXTERNAL-SUBMODULES

pub mod console_template;

// endregion: EXTERNAL-SUBMODULES
