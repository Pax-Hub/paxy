fn main() -> process::ExitCode {
    let return_value = paxy_gui::run_gui();
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
