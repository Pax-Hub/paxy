//! Has the [`run_cli`] function

/// Calls the [`ui::run_common::<C>`] function supplying it with the commandline
///  interface template as a type. Any errors are thrown back to the calling
/// function. A debug message is then displayed conveying that the program is
/// being run in the CLI mode.
pub fn run_cli() -> Result<(), paxy::Error> {
    // Initialize the app to setup basic functions like config and logging
    let (console_input, _logging_worker_guards) =
        app::run_common::<CliTemplate>().context(AppSnafu {})?;

    // Notify that the app is running in CLI mode
    tracing::debug!(
        "Running in {} mode... {}",
        "CLI".blue(),
        console::Emoji("🔤", "")
    );

    // Delegate handling actions
    action::handle_action(console_input).context(ActionSnafu {})?;

    Ok(())
}
// region: IMPORTS

use owo_colors::OwoColorize;
use paxy::{
    action,
    app::{self, ui::console_template::cli::CliTemplate},
    ActionSnafu,
    AppSnafu,
};
use snafu::ResultExt;

// endregion: IMPORTS
