#[tracing::instrument(level = "trace")]
pub fn run_common<C>() -> Result<(C, Vec<WorkerGuard>), crate::Error>
where
    C: clap::Parser + GlobalArguments + fmt::Debug,
{
    // Obtain CLI arguments
    let console_input = C::parse();

    // Obtain user configuration
    let config = config::init_config(&console_input)
        .context(app::ConfigSnafu {})
        .context(crate::AppSnafu)?;

    // Begin logging and outputting
    let logging_handle = logging::init_log(&config)
        .context(app::LoggingSnafu {})
        .context(crate::AppSnafu {})?;

    emit_welcome_messages();

    emit_diagnostic_messages(&config);

    emit_test_messages(&config, &console_input);

    Ok((console_input, logging_handle.worker_guards))
}

fn emit_welcome_messages() {
    // Welcome messages
    tracing::debug!(
        "{} - {}",
        "Paxy".bold(),
        "A package manager that gets out of your way".magenta()
    );
    tracing::debug!(
        "{}  {} {}",
        console::Emoji("✉️", ""),
        "shivanandvp".italic(),
        "<pvshvp.oss@gmail.com, shivanandvp@rebornos.org>".italic()
    );
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

    tracing::debug!(
        "{} {} {:?}",
        console::Emoji("📂", ""),
        "Config Filepath(s):".magenta(),
        config.config_filepaths,
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConsoleOutputMode {
    Regular,
    Plain,
    Json,
    Test,
}

impl Default for ConsoleOutputMode {
    fn default() -> Self {
        ConsoleOutputMode::Regular
    }
}

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

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display(""))]
    Dummy {},
}

// region: IMPORTS

use core::fmt;
use std::path::PathBuf;

use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};
use tracing_appender::non_blocking::WorkerGuard;

use crate::app::{self, config, logging};

// endregion: IMPORTS

// region: MODULES

/// Common commandline interface template for global arguments, intended to be
/// shared between the GUI and CLI programs.
pub mod cli_template {
    #[derive(Clone, Debug, Args)]
    #[command(next_display_order = usize::MAX - 100)]
    pub struct GlobalArgs<L>
    where
        L: clap_verbosity_flag::LogLevel,
    {
        #[arg(
            long = "config",
            short = 'c',
            help = "Path to the configuration file to use.",
            global = true,
            display_order = usize::MAX - 6
        )]
        pub config_file: Option<PathBuf>,

        #[arg(
            long = "json",
            help = "Output in the JSON format for machine readability and scripting purposes.",
            global = true,
            display_order = usize::MAX - 5
        )]
        pub json_flag: bool,

        #[arg(
            long = "plain",
            help = "Output as plain text without extra information, for machine readability and scripting purposes.",
            global = true,
            display_order = usize::MAX - 4
        )]
        pub plain_flag: bool,

        #[arg(
            long = "debug",
            help = "Output debug messages.",
            global = true,
            display_order = usize::MAX - 3
        )]
        pub debug_flag: bool,

        #[arg(
            long = "no-color",
            help = "Disable output coloring.",
            global = true,
            display_order = usize::MAX - 2
        )]
        pub no_color_flag: bool,

        #[arg(
            long = "test",
            help = "Avoid destructive modifications and show all output subject to the commandline filters. Useful for dry-runs and for developers.",
            global = true,
            display_order = usize::MAX - 1
        )]
        pub test_flag: bool,

        #[command(flatten)]
        pub verbosity: clap_verbosity_flag::Verbosity<L>,
    }

    impl<L> GlobalArguments for GlobalArgs<L>
    where
        L: clap_verbosity_flag::LogLevel,
    {
        fn config_filepath(&self) -> &Option<PathBuf> {
            &self.config_file
        }

        fn is_json(&self) -> bool {
            self.json_flag
        }

        fn is_plain(&self) -> bool {
            self.plain_flag
        }

        fn is_debug(&self) -> bool {
            self.debug_flag
        }

        fn is_test(&self) -> bool {
            self.test_flag
        }

        fn is_no_color(&self) -> bool {
            self.no_color_flag
        }

        fn verbosity_filter(&self) -> log::LevelFilter {
            self.verbosity
                .log_level_filter()
        }
    }

    // region: IMPORTS

    use std::path::PathBuf;

    use clap::Args;

    use super::GlobalArguments;

    // endregion: IMPORTS
}

// endregion: MODULES

// region: RE-EXPORTS

#[allow(unused_imports)]
pub use cli_template::*;

use super::config::ConfigTemplate; // Flatten the module heirarchy for easier access

// endregion: RE-EXPORTS
