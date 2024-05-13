//! Has the [`run_gui`] function

/// Calls the [`ui::run_common::<C>`] function supplying it with the
/// commandline  interface template as a type. Any errors are thrown
/// back to the calling function. A debug message is then displayed
/// conveying that the program is being run in the GUI mode.
pub fn run_gui() -> Result<(), paxy::Error> {
    // Initialize the app to setup basic functions like config and logging
    let (_cli_input, _logging_worker_guards) =
        app::run_common::<CliTemplate>().context(AppSnafu {})?;

    // Notify that the app is running in GUI mode
    tracing::debug!(
        "Running in {} mode... {}",
        "GUI".blue(),
        console::Emoji("📊", "")
    );

    todo!("GUI is not yet implemented");

    // Ok(())
}

// region: IMPORTS

use owo_colors::OwoColorize;
use paxy::{
    app::{self, ui::console_template::gui::CliTemplate},
    AppSnafu,
};
use snafu::ResultExt;

// endregion: IMPORTS
