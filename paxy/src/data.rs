// TODO: Write code here

// region: ERRORS

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

// region: ERRORS

// region: IMPORTS

// use std::{fmt, str::FromStr};

// use serde::{Deserialize, Serialize};
// use serde_aux::prelude::*;
// use speedy::{Readable, Writable};
use snafu::Snafu;

// endregion: IMPORTS

// region: EXTERNAL-SUBMODULES

// pub mod some_module;
mod config;

// endregion: EXTERNAL-SUBMODULES

