pub fn type_of<T>(_: &T) -> &str {
    any::type_name::<T>()
}

// region: IMPORTS

use std::any;

use snafu::Snafu;

// endregion: IMPORTS

// region: ERRORS

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display("in the application: {source}"), visibility(pub))]
    App {
        #[snafu(backtrace)]
        source: app::Error,
    },

    #[non_exhaustive]
    #[snafu(display("in an action:{source}"), visibility(pub))]
    Actions {
        #[snafu(backtrace)]
        source: actions::Error,
    },
}

// endregion: ERRORS

// region: EXTERNAL-SUBMODULES

pub mod actions;
pub mod app;
pub mod data;

// endregion: EXTERNAL-SUBMODULES
