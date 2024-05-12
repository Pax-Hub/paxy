// Returns a string representation of the type of the given object, which can
// be displayed or further processed.
pub fn type_of<T>(_: &T) -> &str {
    any::type_name::<T>()
}

// region: ERRORS

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display("in the app: {source}"), visibility(pub))]
    App {
        #[snafu(backtrace)]
        source: app::Error,
    },

    #[non_exhaustive]
    #[snafu(display("in an action:{source}"), visibility(pub))]
    Action {
        #[snafu(backtrace)]
        source: action::Error,
    },
}

// endregion: ERRORS

// region: IMPORTS

use std::any;

use snafu::Snafu;

// endregion: IMPORTS

// region: EXTERNAL-SUBMODULES

pub mod action;
pub mod app;
pub mod data;

// endregion: EXTERNAL-SUBMODULES
