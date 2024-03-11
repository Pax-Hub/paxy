lazy_static! {
    pub static ref APP_NAME: &'static str = "paxy";
}

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
    #[snafu(display("in internationalization: {source}"), visibility(pub))]
    Internationalization {
        #[snafu(backtrace)]
        source: i18n::Error,
    },
}

// region: IMPORTS
use lazy_static::lazy_static;
use snafu::Snafu;

// endregion: IMPORTS

// region: MODULES

pub mod config;
pub mod i18n;
pub mod logging;

// endregion: MODULES

// region: RE-EXPORTS

#[allow(unused_imports)]
pub use config::*;
#[allow(unused_imports)]
pub use i18n::*;
#[allow(unused_imports)]
pub use logging::*;

// endregion: RE-EXPORTS
