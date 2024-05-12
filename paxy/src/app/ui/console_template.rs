//! Common commandline interface template for global arguments, intended to be
//! shared between the GUI and CLI programs.
#[derive(Clone, Debug, Args)]
#[command(next_display_order = usize::MAX - 100)]
pub struct GlobalArgs<L>
where
    L: clap_verbosity_flag::LogLevel,
{
    #[arg(
            long = "config",
            short = 'c',
            help = "Path to the configuration file to use.",
            global = true,
            display_order = usize::MAX - 6
        )]
    pub config_file: Option<PathBuf>,

    #[arg(
            long = "json",
            help = "Output in the JSON format for machine readability and scripting purposes.",
            global = true,
            display_order = usize::MAX - 5
        )]
    pub json_flag: bool,

    #[arg(
            long = "plain",
            help = "Output as plain text without extra information, for machine readability and scripting purposes.",
            global = true,
            display_order = usize::MAX - 4
        )]
    pub plain_flag: bool,

    #[arg(
            long = "debug",
            help = "Output debug messages.",
            global = true,
            display_order = usize::MAX - 3
        )]
    pub debug_flag: bool,

    #[arg(
            long = "no-color",
            help = "Disable output coloring.",
            global = true,
            display_order = usize::MAX - 2
        )]
    pub no_color_flag: bool,

    #[arg(
            long = "test",
            help = "Avoid destructive modifications and show all output subject to the commandline filters. Useful for dry-runs and for developers.",
            global = true,
            display_order = usize::MAX - 1
        )]
    pub test_flag: bool,

    #[command(flatten)]
    pub verbosity: clap_verbosity_flag::Verbosity<L>,
}

impl<L> GlobalArguments for GlobalArgs<L>
where
    L: clap_verbosity_flag::LogLevel,
{
    fn config_filepath(&self) -> &Option<PathBuf> {
        &self.config_file
    }

    fn is_json(&self) -> bool {
        self.json_flag
    }

    fn is_plain(&self) -> bool {
        self.plain_flag
    }

    fn is_debug(&self) -> bool {
        self.debug_flag
    }

    fn is_test(&self) -> bool {
        self.test_flag
    }

    fn is_no_color(&self) -> bool {
        self.no_color_flag
    }

    fn verbosity_filter(&self) -> log::LevelFilter {
        self.verbosity
            .log_level_filter()
    }
}

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display(""), visibility(pub))]
    GuiDummy {}, // No errors implemented yet
}

// region: IMPORTS

// endregion: IMPORTS

// region: MODULES

pub mod cli;
pub mod gui;

// endregion: MODULES

// region: IMPORTS
use std::path::PathBuf;

use clap::Args;
use owo_colors::OwoColorize;
use paxy::app::ui;
use snafu::Snafu;

use super::GlobalArguments;

// endregion: IMPORTS
