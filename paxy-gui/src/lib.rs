//! Has the [`run_gui`] function and the commandline interface template
//! [`gui_cli_template::CliTemplate`]

/// Calls the [`ui::run_common::<C>`] function supplying it with the commandline
///  interface template as a type. Any errors are thrown back to the calling
/// function. A debug message is then displayed conveying that the program is
/// being run in the GUI mode.
pub fn run_gui() -> Result<(), paxy::Error> {
    let (_cli_input, _logging_worker_guards) = ui::run_common::<CliTemplate>()?;

    tracing::debug!(
        "Running in {} mode... {}",
        "GUI".blue(),
        console::Emoji("📊", "")
    );

    Ok(())
}

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display(""), visibility(pub))]
    GuiDummy {}, // No errors implemented yet
}

// region: IMPORTS

use owo_colors::OwoColorize;
use paxy::ui;
use snafu::Snafu;

// endregion: IMPORTS

// region: MODULES

/// The commandline interface for the GUI program. Allows one to specify flags
/// that control output on a console.
mod gui_cli_template {

    /// The base commandline template consists of global arguments
    #[derive(Parser, Debug)]
    #[command(version, author, about, args_conflicts_with_subcommands = true)]
    pub struct CliTemplate {
        #[clap(flatten)]
        pub global_args: ui::cli_template::GlobalArgs<clap_verbosity_flag::InfoLevel>,
    }

    /// Implement a trait that can extract standard global arguments from our
    /// own CLI template
    impl ui::GlobalArguments for CliTemplate {
        type L = clap_verbosity_flag::InfoLevel;

        fn config_file(&self) -> &Option<PathBuf> {
            &self
                .global_args
                .config_file
        }

        fn is_json(&self) -> bool {
            self.global_args
                .json_flag
        }

        fn is_plain(&self) -> bool {
            self.global_args
                .plain_flag
        }

        fn is_debug(&self) -> bool {
            self.global_args
                .debug_flag
        }

        fn is_no_color(&self) -> bool {
            self.global_args
                .no_color_flag
        }

        fn is_test(&self) -> bool {
            self.global_args
                .test_flag
        }

        fn verbosity(&self) -> &clap_verbosity_flag::Verbosity<Self::L> {
            &self
                .global_args
                .verbose
        }
    }

    // region: IMPORTS

    use std::path::PathBuf;

    use clap::Parser;
    use paxy::ui;

    // endregion: IMPORTS
}

// endregion: MODULES

// region: RE-EXPORTS

pub use gui_cli_template::*; // Flatten the module heirarchy for easier access

// endregion: RE-EXPORTS
