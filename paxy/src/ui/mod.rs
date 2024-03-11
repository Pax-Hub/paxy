#[tracing::instrument(level = "trace")]
pub fn run_common<C>() -> Result<(C, Vec<WorkerGuard>), crate::Error>
where
    C: clap::Parser + CliModifier + fmt::Debug,
    <C as GlobalArguments>::L: LogLevel,
{
    // Obtain user configuration
    let (config, config_filepaths) = config::init_config()
        .context(app::ConfigSnafu {})
        .context(crate::AppSnafu)?;

    // Obtain CLI arguments
    let cli_input = C::parse();

    // Turn off colors if needed
    let mut is_cli_uncolored = cli_input.is_uncolored();
    if !is_cli_uncolored {
        if let Some(no_color) = config.no_color {
            is_cli_uncolored = no_color;
        }
    }
    if is_cli_uncolored {
        anstream::ColorChoice::Never.write_global();
        owo_colors::set_override(false);
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
    let verbosity_filter = cli_input
        .verbosity_filter()
        .or(config_verbosity_filter);
    let (mut handle, log_filepath) = logging::init_log(config_log_dirpath, verbosity_filter)
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

    // Welcome message
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

    // Test messages
    tracing::trace!(target:"TEST", "{} Testing trace!...", console::Emoji("🧪", ""));
    tracing::debug!(target:"TEST", "{} Testing debug!...", console::Emoji("🧪", ""));
    tracing::info!(target:"TEST", "{} Testing info!...", console::Emoji("🧪", ""));
    tracing::warn!(target:"TEST", "{} Testing warn!...", console::Emoji("🧪", ""));
    tracing::error!(target:"TEST", "{} Testing error!...", console::Emoji("🧪", ""));

    tracing::info!(target:"JSON", "{} Testing: {}", console::Emoji("🧪", ""), "{\"JSON\": \"Target\"}");
    tracing::info!(target:"PLAIN", "{} Testing: Plain Target", console::Emoji("🧪", ""));

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

    Ok((cli_input, handle.worker_guards))
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
            return Some(LevelFilter::INFO);
        }

        let verbosity_flag_filter = self
            .verbosity()
            .log_level_filter();

        if verbosity_flag_filter < clap_verbosity_flag::LevelFilter::Debug && self.is_debug() {
            return Some(LevelFilter::DEBUG);
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
use tracing_subscriber::filter::LevelFilter;

use crate::app::{self, config, logging};

// endregion: IMPORTS

// region: MODULES

mod cli_template {
    #[derive(Clone, Debug, Args)]
    #[clap(args_conflicts_with_subcommands = true, next_display_order = usize::MAX - 100)]
    pub struct GlobalArgs<L>
    where
        L: clap_verbosity_flag::LogLevel,
    {
        #[clap(
            long = "config",
            short = 'c',
            help = "Path to the configuration file to use.",
            global = true,
            display_order = usize::MAX - 6
        )]
        pub config_file: Option<PathBuf>,

        #[clap(
            long = "json",
            help = "Output in the JSON format for machine readability and scripting purposes.",
            global = true,
            display_order = usize::MAX - 5
        )]
        pub json_flag: bool,

        #[clap(
            long = "plain",
            help = "Output as plain text without extra information, for machine readability and scripting purposes.",
            global = true,
            display_order = usize::MAX - 4
        )]
        pub plain_flag: bool,

        #[clap(
            long = "debug",
            help = "Output debug messages.",
            global = true,
            display_order = usize::MAX - 3
        )]
        pub debug_flag: bool,

        #[clap(
            long = "no-color",
            help = "Disable output coloring.",
            global = true,
            display_order = usize::MAX - 2
        )]
        pub no_color_flag: bool,

        #[clap(
            long = "test",
            help = "Avoid destructive modifications and show all output subject to the commandline filters. Useful for dry-runs and for developers.",
            global = true,
            display_order = usize::MAX - 1
        )]
        pub test_flag: bool,

        #[clap(flatten)]
        pub verbose: Verbosity<L>,
    }

    // region: IMPORTS

    use std::path::PathBuf;

    use clap::Args;
    use clap_verbosity_flag::Verbosity;

    // endregion: IMPORTS
}

// endregion: MODULES

// region: RE-EXPORTS

pub use cli_template::*;

// endregion: RE-EXPORTS
