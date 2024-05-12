//! Has the [`run_gui`] function

/// Calls the [`ui::run_common::<C>`] function supplying it with the
/// commandline  interface template as a type. Any errors are thrown
/// back to the calling function. A debug message is then displayed
/// conveying that the program is being run in the GUI mode.
pub fn run_gui() -> Result<(), paxy::Error> {
    let (_cli_input, _logging_worker_guards) = paxy::ui::run_common::<CliTemplate>()?;

    tracing::debug!(
        "Running in {} mode... {}",
        "GUI".blue(),
        console::Emoji("📊", "")
    );

    Ok(())
}
