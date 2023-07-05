#![warn(missing_docs)]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/PaxyHub/.github/main/paxy_logo.png")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/PaxyHub/.github/main/paxy_logo.svg")]

//! The main binary crate which contains the commandline interface used to
//! interact with the paxy program while it is running.

mod cli;
mod config;

use cli::CommandlineDispatcher;
use snafu::Whatever;

/// The main function where the program execution begins when `paxy` is run.
#[snafu::report]
fn main() -> Result<(), Whatever> {
    CommandlineDispatcher::run()?;
    Ok(())
}
