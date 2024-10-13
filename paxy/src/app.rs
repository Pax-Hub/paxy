//! Handles application related functionality - UI, logging, config, OS, etc.
//! , not the various package or repository actions that form the core
//! functionality of paxy.

lazy_static! {
    /// A global variable that represents the application name.
    pub static ref APP_NAME: &'static str = "paxy";
}

/// Run common tasks pertaining to both CLI and GUI. This includes parsing
/// console arguments, obtaining user configuration
#[tracing::instrument(level = "trace")]
pub fn run_common<C>() -> Result<(C, Vec<tracing_appender::non_blocking::WorkerGuard>), Error>
where
    // [`clap::Parser`] binding to parse console input, [`GlobalArguments`]
    // binding to extract global arguments, and [`fmt::Debug`] binding to
    // display commandline arguments
    C: clap::Parser + GlobalArguments + fmt::Debug,
{
    // Obtain CLI arguments. Also provides info for setting up configuration and
    // logging
    let console_input = C::parse();

    // Obtain user configuration
    let config = config::init_config(&console_input).context(ConfigSnafu {})?;

    let i18n_handle = i18n::init_i18n(&config).context(I18nSnafu {})?;

    // Begin logging and outputting to console
    let logging_handle = logging::init_log(&config).context(LoggingSnafu {})?;

    // Display initializing messages
    ui::emit_init_messages(&config, &console_input);

    Ok((console_input, logging_handle.worker_guards))
}

// region: ERRORS

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display("in logging: {source}"), visibility(pub))]
    Logging {
        #[snafu(backtrace)]
        source: logging::Error,
    },

    #[non_exhaustive]
    #[snafu(display("in the configuration: {source}"), visibility(pub))]
    Config {
        #[snafu(backtrace)]
        source: config::Error,
    },

    #[non_exhaustive]
    #[snafu(
        display("in internationalization/regionalization/translation: {source}"),
        visibility(pub)
    )]
    I18n {
        #[snafu(backtrace)]
        source: i18n::Error,
    },
}

// endregion: ERRORS

// region: IMPORTS

use std::fmt;

use lazy_static::lazy_static;
use snafu::{ResultExt, Snafu};
use ui::GlobalArguments;

// endregion: IMPORTS

// region: EXTERNAL-SUBMODULES

pub mod config;
pub mod i18n;
pub mod logging;
pub mod ui;

// endregion: EXTERNAL-SUBMODULES
