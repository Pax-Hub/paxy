//! The commandline interface for the GUI program. Allows one to specify
//! flags that control output on a console.

/// The base commandline template consists of global arguments
#[derive(Parser, Debug)]
#[command(version, author, about, args_conflicts_with_subcommands = true)]
pub struct CliTemplate {
    #[clap(flatten)]
    pub global_args: GlobalArgs<clap_verbosity_flag::InfoLevel>,
}

/// Implement a trait that can extract standard global arguments from
/// our own CLI template
impl ui::GlobalArguments for CliTemplate {
    fn config_filepath(&self) -> &Option<PathBuf> {
        self.global_args
            .config_filepath()
    }

    fn is_json(&self) -> bool {
        self.global_args
            .is_json()
    }

    fn is_plain(&self) -> bool {
        self.global_args
            .is_plain()
    }

    fn is_debug(&self) -> bool {
        self.global_args
            .is_debug()
    }

    fn is_test(&self) -> bool {
        self.global_args
            .is_test()
    }

    fn is_no_color(&self) -> bool {
        self.global_args
            .is_no_color()
    }

    fn verbosity_filter(&self) -> log::LevelFilter {
        self.global_args
            .verbosity_filter()
    }
}

// region: IMPORTS

use std::path::PathBuf;

use clap::Parser;

use crate::app::ui::{self, console_template::GlobalArgs};

// endregion: IMPORTS
