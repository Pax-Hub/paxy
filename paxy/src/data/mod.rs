#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(
        display("'{value}' is not recognized as a variant of '{enum_name}'. The allowed values are {allowed_values:?}")
    )]
    InvalidVariant {
        value: String,
        enum_name: String,
        allowed_values: Vec<String>,
    },
}

// region: IMPORTS

// use std::{fmt, str::FromStr};

// use serde::{Deserialize, Serialize};
// use serde_aux::prelude::*;
use snafu::Snafu;
// use speedy::{Readable, Writable};

// endregion: IMPORTS

// region: MODULES

// pub mod some_module;

// endregion: MODULES

// region: RE-EXPORTS

// #[allow(unused_imports)]
// pub use some_module::*;

// endregion: RE-EXPORTS
