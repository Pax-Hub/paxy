pub fn run_cli() -> Result<(), paxy::Error> {
    let (_cli_input, _worker_guards) = ui::run_common::<CliTemplate>()?;

    tracing::debug!(
        "Running in {} mode... {}",
        "CLI".blue(),
        console::Emoji("🔤", "")
    );

    Ok(())
}

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display(""), visibility(pub))]
    CliDummy {},
}

// region: IMPORTS
use owo_colors::OwoColorize;
use paxy::ui;
use snafu::Snafu;

// endregion: IMPORTS

// region: MODULES

/// Groups together all the data structures that make up the `derive` interface
/// of the `clap` library. This is separate from any of the helper functions
/// that are used as part of the commandline interface.
mod cli_template {
    #[derive(Parser, Debug)]
    #[command(version, author, about, args_conflicts_with_subcommands = true)]
    pub struct CliTemplate {
        #[clap(flatten)]
        pub global_args: GlobalArgs<clap_verbosity_flag::InfoLevel>,

        #[clap(subcommand)]
        pub command: Option<ActionCommand>,

        #[clap(flatten)]
        pub arguments: ListActionArguments,
    }

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

    #[derive(Debug, Subcommand)]
    #[clap(args_conflicts_with_subcommands = true)]
    pub enum ActionCommand {
        #[clap(name = "list", about = "List installed packages.", display_order = 1)]
        List(ListActionArguments),

        #[clap(
            name = "search",
            alias = "find",
            about = "Search for available packages.",
            display_order = 2
        )]
        Search(SearchActionArguments),

        #[clap(
            name = "install",
            alias = "add",
            about = "Install packages.",
            display_order = 3
        )]
        Install(InstallActionArguments),

        #[clap(
            name = "update",
            alias = "upgrade",
            about = "Update packages.",
            display_order = 4
        )]
        Update(UpdateActionArguments),

        #[clap(
            name = "uninstall",
            alias = "remove",
            about = "Uninstall packages.",
            display_order = 5
        )]
        Uninstall(UninstallActionArguments),

        #[clap(
            name = "downgrade",
            about = "Downgrade a package.",
            display_order = 5
        )]
        Downgrade(DowngradeActionArguments),
    }

    #[derive(Debug, Args)]
    pub struct ListActionArguments {
        #[clap(
            long = "exclude",
            alias = "ignore",
            short = 'e',
            help = "Partial or full name(s) of packages to exclude from the search among the installed packages.",
            display_order = 1
        )]
        pub excluded_partial_package_names: Vec<String>,

        #[clap(
            help = "Partial or full name(s) of the packages to search among the installed packages. Not specifying this argument will list all packages.",
            display_order = usize::MAX - 1
        )]
        pub partial_package_name: Option<String>, // This should always be the last argument
    }

    #[derive(Debug, Args)]
    pub struct SearchActionArguments {
        #[clap(
            long = "exclude",
            alias = "ignore",
            short = 'e',
            help = "Partial or full name(s) of packages to exclude from the search among available packages.",
            display_order = 1
        )]
        pub excluded_partial_package_names: Vec<String>,

        #[clap(
            help = "Partial or full name(s) of the packages to search among available packages.",
            display_order = usize::MAX - 1
        )]
        pub partial_package_name: String, // This should always be the last argument
    }

    #[derive(Debug, Args)]
    pub struct InstallActionArguments {
        #[clap(help = "Full name(s) of the packages to install.", display_order = usize::MAX - 1)]
        pub package_names: Vec<String>, // This should always be the last argument
    }

    #[derive(Debug, Args)]
    pub struct UpdateActionArguments {
        #[clap(
            long = "exclude",
            alias = "ignore",
            short = 'e',
            help = "Full name(s) of packages to exclude from updating.",
            display_order = 1
        )]
        pub excluded_package_names: Vec<String>,

        #[clap(
            help = "Full name(s) of the packages to update. Not specifying this argument will update all packages",
            display_order = usize::MAX - 1
        )]
        pub package_names: Vec<String>, // This should always be the last argument
    }

    #[derive(Debug, Args)]
    pub struct UninstallActionArguments {
        #[clap(
            help = "Full name(s) of the packages to uninstall.",
            display_order = usize::MAX - 1
        )]
        pub package_names: Vec<String>, // This should always be the last argument
    }

    #[derive(Debug, Args)]
    pub struct DowngradeActionArguments {
        #[clap(
            long = "version",
            alias = "ver",
            help = "The version to downgrade to.",
            display_order = 1
        )]
        pub version: Option<String>,

        #[clap(
            help = "Full name of the package to downgrade.",
            display_order = usize::MAX - 1
        )]
        pub package_name: String, // This should always be the last argument
    }

    // region: IMPORTS

    use std::path::PathBuf;

    use clap::{Args, Parser, Subcommand};
    use paxy::ui::{self, GlobalArgs};

    // endregion: IMPORTS
}

// endregion: MODULES

// region: RE-EXPORTS

pub use cli_template::*;

// endregion: RE-EXPORTS
