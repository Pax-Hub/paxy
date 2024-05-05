lazy_static! {
    pub static ref APP_NAME: &'static str = "paxy";
}

// region: IMPORTS

use std::path::PathBuf;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use snafu::Snafu;

// endregion: IMPORTS

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
    #[snafu(display("in configuration: {source}"), visibility(pub))]
    Config {
        #[snafu(backtrace)]
        source: config::Error,
    },

    #[non_exhaustive]
    #[snafu(display("in the UI: {source}"), visibility(pub))]
    Ui {
        #[snafu(backtrace)]
        source: ui::Error,
    },

    #[non_exhaustive]
    #[snafu(display("in internationalization: {source}"), visibility(pub))]
    Internationalization {
        #[snafu(backtrace)]
        source: i18n::Error,
    },
}

// endregion: ERRORS

// region: EXTERNAL-SUBMODULES

pub mod config;
pub mod i18n;
pub mod logging;
pub mod ui;

// endregion: EXTERNAL-SUBMODULES
