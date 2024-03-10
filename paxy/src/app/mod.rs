lazy_static! {
    pub static ref APP_NAME: &'static str = "paxy";
}

pub trait PathListPermissions<'a, P>: Iterator<Item = P> + Sized
where
    P: AsRef<Path> + 'a,
{
    fn first_readable_path(mut self) -> Option<P> {
        self.find(|p| permissions::is_readable(p).unwrap_or(false))
    }

    fn first_writable_path(mut self) -> Option<P> {
        self.find(|p| permissions::is_writable(p).unwrap_or(false))
    }

    fn all_readable_paths(self) -> iter::Filter<Self, fn(&P) -> bool> {
        self.filter(|p| permissions::is_readable(p).unwrap_or(false))
    }

    fn all_writable_paths(self) -> iter::Filter<Self, fn(&P) -> bool> {
        self.filter(|p| permissions::is_writable(p).unwrap_or(false))
    }
}

impl<'a, T, P> PathListPermissions<'a, P> for T
where
    T: Iterator<Item = P>,
    P: AsRef<Path> + 'a,
{
}

pub fn first_readable_path<'a>(
    paths: &'a Vec<impl AsRef<Path> + 'a>,
) -> Option<impl AsRef<Path> + 'a> {
    paths
        .iter()
        .find(|p| permissions::is_readable(p).unwrap_or(false))
}

pub fn first_writable_path<'a>(
    paths: &'a Vec<impl AsRef<Path> + 'a>,
) -> Option<impl AsRef<Path> + 'a> {
    paths
        .iter()
        .find(|p| permissions::is_writable(p).unwrap_or(false))
}

pub fn all_readable_paths<'a>(
    paths: &'a Vec<impl AsRef<Path> + 'a>,
) -> impl Iterator<Item = impl AsRef<Path> + 'a> {
    paths
        .iter()
        .filter(|p| permissions::is_readable(p).unwrap_or(false))
}

pub fn all_writable_paths<'a>(
    paths: &'a Vec<impl AsRef<Path> + 'a>,
) -> impl Iterator<Item = impl AsRef<Path> + 'a> {
    paths
        .iter()
        .filter(|p| permissions::is_writable(p).unwrap_or(false))
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

use std::{iter, path::Path};

use lazy_static::lazy_static;
use snafu::Snafu;

// endregion: IMPORTS

// region: MODULES

pub mod config;
pub mod i18n;
pub mod logging;

// endregion: MODULES

// region: RE-EXPORTS

pub use config::*;
pub use i18n::*;
pub use logging::*;

// endregion: RE-EXPORTS
