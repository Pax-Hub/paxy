//! Starts execution at the [`main`] function. Offloads the implemenation
//! details to its corresponding library crate.

/// Calls the [`crate::run_cli`] function and captures the returned
/// [`Result`]. If there was an error, the error message chain is printed to the
///  standard error stream (`stderr`). The program then returns an `0` or `1`
/// corresponding to "no error" or "error" based on the result.
fn main() -> process::ExitCode {
    let return_value = crate::run_cli();
    match return_value {
        Ok(_) => process::ExitCode::from(0),
        Err(err_value) => {
            anstream::eprintln!(
                "{} {err_value}",
                "[ERROR]"
                    .bold()
                    .red()
            );
            process::ExitCode::from(1)
        }
    }
}

// region: IMPORTS

use std::process;

use owo_colors::OwoColorize;

// endregion: IMPORTS
