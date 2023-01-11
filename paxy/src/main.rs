#![warn(missing_docs)]

//! The main binary crate which contains the commandline interface used to
//! interact with the program while it is running.

mod cli;
mod package;

use cli::CommandlineDispatcher;

/// The main function where the program execution begins when `paxy` is run.
fn main() {
    CommandlineDispatcher::run();
}
