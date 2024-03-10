pub trait InitI18n {}

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display(""), visibility(pub))]
    I18nDummy {},
}

// region: IMPORTS

use snafu::Snafu;

// endregion: IMPORTS
