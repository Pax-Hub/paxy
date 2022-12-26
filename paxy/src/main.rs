#![warn(missing_docs)]

//! The main binary crate which contains the commandline interface used to
//! interact with the program while it is running

mod cli;

/// The main function where the program execution begins when `paxy` is run
fn main() {
    let commandline_dispatcher: cli::CommandlineDispatcher = cli::CommandlineDispatcher::new();
    commandline_dispatcher.run();
}
