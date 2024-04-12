#[tracing::instrument(level = "trace")]
pub fn run_common<C>() -> Result<(C, Vec<WorkerGuard>), crate::Error>
where
    C: clap::Parser + CliModifier + fmt::Debug,
    <C as GlobalArguments>::L: LogLevel,
{
    // Obtain CLI arguments
    let cli_input = C::parse();

    // Obtain user configuration
    let (config, config_filepaths) = config::init_config(&cli_input)
        .context(app::ConfigSnafu {})
        .context(crate::AppSnafu)?;

    // Turn off colors if needed
    if let Some(no_color) = config.no_color {
        if no_color {
            anstream::ColorChoice::Never.write_global();
            owo_colors::set_override(false);
        }
    }

    // Begin logging with preferred log directory and preferred verbosity
    let config_log_dirpath = config
        .log_directory
        .as_ref()
        .map(PathBuf::from);
    let config_verbosity_filter: Option<LevelFilter> = config
        .log_level_filter
        .and_then(|lf| {
            lf.as_str()
                .parse()
                .ok()
        });
    let (mut handle, log_filepath) = logging::init_log(config_log_dirpath, config_verbosity_filter.into())
        .context(app::LoggingSnafu {})
        .context(crate::AppSnafu {})?;

    // Modify logging behavior if Plain or Json output is desired
    if cli_input.is_json() {
        handle
            .switch_to_json()
            .context(app::LoggingSnafu {})
            .context(crate::AppSnafu {})?;
    } else if cli_input.is_plain() {
        handle
            .switch_to_plain()
            .context(app::LoggingSnafu {})
            .context(crate::AppSnafu {})?;
    } else if cli_input.is_test() {
        handle
            .switch_to_test()
            .context(app::LoggingSnafu {})
            .context(crate::AppSnafu {})?;
    }

    emit_welcome_messages();
    emit_diagnostic_messages(config_filepaths, log_filepath, &cli_input);
    emit_test_messages();

    Ok((cli_input, handle.worker_guards))
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

fn emit_diagnostic_messages<C>(config_filepaths: Vec<PathBuf>, log_filepath: PathBuf, cli_input: &C)
where
    C: clap::Parser + CliModifier + fmt::Debug,
    <C as GlobalArguments>::L: LogLevel,
{
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
        "Config Filepath(s) (without file extensions):".magenta(),
        config_filepaths,
    );
    tracing::debug!(
        "{} {} {:?}",
        console::Emoji("📂", ""),
        "Log Filepath:".magenta(),
        log_filepath
    );

    tracing::trace!(
        "{}  {} {:#?}",
        console::Emoji("⌨️", ""),
        "CLI input arguments:"
            .magenta()
            .dimmed(),
        cli_input.dimmed()
    );
}

fn emit_test_messages() {
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
}

impl<T> CliModifier for T
where
    T: GlobalArguments,
    <T as GlobalArguments>::L: LogLevel,
{
}

pub trait CliModifier: GlobalArguments
where
    <Self as GlobalArguments>::L: LogLevel,
{
    fn verbosity_filter(&self) -> Option<LevelFilter> {
        if self.is_plain() || self.is_json() {
            return Some(LevelFilter::Info);
        }

        let verbosity_flag_filter = self
            .verbosity()
            .log_level_filter();

        if verbosity_flag_filter < clap_verbosity_flag::LevelFilter::Debug && self.is_debug() {
            return Some(LevelFilter::Debug);
        }

        verbosity_flag_filter
            .as_str()
            .parse()
            .ok()
    }

    fn is_uncolored(&self) -> bool {
        self.is_plain()
            || self.is_json()
            || self.is_no_color()
            || env::var(format!(
                "{}_NO_COLOR",
                String::from(*app::APP_NAME).to_uppercase()
            ))
            .map_or(false, |value| !value.is_empty())
    }

    fn is_colored(&self) -> bool {
        !self.is_uncolored()
    }
}

pub trait GlobalArguments {
    type L;

    fn config_file(&self) -> &Option<PathBuf>;

    fn is_json(&self) -> bool;

    fn is_plain(&self) -> bool;

    fn is_debug(&self) -> bool;

    fn is_no_color(&self) -> bool;

    fn is_test(&self) -> bool;

    fn verbosity(&self) -> &clap_verbosity_flag::Verbosity<Self::L>
    where
        Self::L: LogLevel;
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
use std::{env, path::PathBuf};

use clap_verbosity_flag::LogLevel;
use owo_colors::OwoColorize;
use snafu::{ResultExt, Snafu};
use tracing_appender::non_blocking::WorkerGuard;
use log::LevelFilter;

use crate::app::{self, config, logging};

// endregion: IMPORTS

// region: MODULES

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
        pub verbose: clap_verbosity_flag::Verbosity<L>,
    }

    // region: IMPORTS

    use std::path::PathBuf;

    use clap::Args;

    // endregion: IMPORTS
}

// endregion: MODULES

// region: RE-EXPORTS

#[allow(unused_imports)]
pub use cli_template::*;

// endregion: RE-EXPORTS
