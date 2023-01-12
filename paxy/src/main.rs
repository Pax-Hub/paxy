#![warn(missing_docs)]

//! The main binary crate which contains the commandline interface used to
//! interact with the program while it is running.

mod cli;
mod config;
mod package;

use cli::CommandlineDispatcher;
use snafu::Whatever;

/// The main function where the program execution begins when `paxy` is run.
#[snafu::report]
fn main() -> Result<(), Whatever> {
    CommandlineDispatcher::run()?;
    Ok(())
}
