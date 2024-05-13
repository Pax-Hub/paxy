//! Translation/Internationalization/Regionalization.

// TODO: Add code here

// region: ERRORS

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display(""), visibility(pub))]
    I18nDummy {},
}

// endregion: ERRORS

// region: IMPORTS

use snafu::Snafu;

// endregion: IMPORTS
