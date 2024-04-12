// TODO: The module code goes here

// region: IMPORTS

use snafu::Snafu;

// endregion: IMPORTS

// region: ERRORS

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display(""), visibility(pub))]
    I18nDummy {},
}

// endregion: ERRORS