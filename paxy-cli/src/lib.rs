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

/// This module is a [*derive* interface template](https://docs.rs/clap/latest/clap/_derive/_tutorial/chapter_0/index.html) specifically for
/// use with the `clap` library. Any other commandline-related code that is not
/// part of the `clap` derive template will not be in this module.
/// The CLI is designed to (as much as possible,) follow the guidelines in
/// https://clig.dev/ . As a consequence, the command structure follows the
/// 'application_name noun verb' order of subcommands. For example:
/// `paxy package list [args]`, `paxy repo add [args]`
mod cli_template {

    /// The base commandline template consists of global arguments, a subcommand
    /// that denotes the entity that is being operated upon (like a package or
    /// repository), and optionally, arguments for the default subcommand (in
    /// this case, the 'package' entity is assumed chosen to act on, by
    /// default).
    #[derive(Debug, Parser)]
    #[command(
        version,
        author,
        about,
        args_conflicts_with_subcommands = true,
        propagate_version = true
    )]
    pub struct CliTemplate {
        #[command(flatten)]
        pub global_arguments: GlobalArguments<clap_verbosity_flag::InfoLevel>,

        #[command(subcommand)]
        pub entity: Option<EntitySubcommand>,
    }

    /// Implement a trait that can extract standard global arguments from our
    /// own CLI template
    impl ui::GlobalArguments for CliTemplate {
        type L = clap_verbosity_flag::InfoLevel;

        fn config_file(&self) -> &Option<PathBuf> {
            &self
                .global_arguments
                .config_file
        }

        fn is_json(&self) -> bool {
            self.global_arguments
                .json_flag
        }

        fn is_plain(&self) -> bool {
            self.global_arguments
                .plain_flag
        }

        fn is_debug(&self) -> bool {
            self.global_arguments
                .debug_flag
        }

        fn is_no_color(&self) -> bool {
            self.global_arguments
                .no_color_flag
        }

        fn is_test(&self) -> bool {
            self.global_arguments
                .test_flag
        }

        fn verbosity(&self) -> &clap_verbosity_flag::Verbosity<Self::L> {
            &self
                .global_arguments
                .verbose
        }
    }

    #[derive(Clone, Debug, Args)]
    #[command(next_display_order = usize::MAX - 100)]
    pub struct GlobalArguments<L>
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
        pub verbose: clap_verbosity_flag::Verbosity<L>,
    }

    #[derive(Debug, Subcommand)]
    #[command(args_conflicts_with_subcommands = true)]
    pub enum EntitySubcommand {
        #[command(
            name = "package",
            about = "Perform actions on package(s).",
            subcommand,
            display_order = 1
        )]
        Package(PackageSubcommand),

        #[command(
            subcommand,
            name = "repository",
            alias = "repo",
            about = "Perform actions on repository(-ies).",
            display_order = 2
        )]
        Repository(RepositorySubcommand),
    }

    #[derive(Debug, Subcommand)]
    #[command(args_conflicts_with_subcommands = true)]
    pub enum PackageSubcommand {
        #[command(name = "list", about = "List installed packages.", display_order = 1)]
        List(PackageListArguments),

        #[command(
            name = "search",
            alias = "find",
            about = "Search for available packages.",
            display_order = 2
        )]
        Search(PackageSearchArguments),

        #[command(
            name = "install",
            alias = "add",
            about = "Install packages.",
            display_order = 3
        )]
        Install(PackageInstallArguments),

        #[command(
            name = "update",
            alias = "upgrade",
            about = "Update packages.",
            display_order = 4
        )]
        Update(PackageUpdateArguments),

        #[command(
            name = "uninstall",
            alias = "remove",
            about = "Uninstall packages.",
            display_order = 5
        )]
        Uninstall(PackageUninstallArguments),

        #[command(name = "downgrade", about = "Downgrade a package.", display_order = 5)]
        Downgrade(PackageDowngradeArguments),
    }

    #[derive(Debug, Subcommand)]
    #[command(args_conflicts_with_subcommands = true)]
    pub enum RepositorySubcommand {
        #[command(
            name = "list",
            about = "List installed repositories.",
            display_order = 1
        )]
        List(RepositoryListArguments),

        #[command(
            name = "search",
            alias = "find",
            about = "Search for available repositories.",
            display_order = 2
        )]
        Search(RepositorySearchArguments),

        #[command(
            name = "install",
            alias = "add",
            about = "Install repositories.",
            display_order = 3
        )]
        Install(RepositoryInstallArguments),

        #[command(
            name = "update",
            alias = "upgrade",
            about = "Update repositories.",
            display_order = 4
        )]
        Update(RepositoryUpdateArguments),

        #[command(
            name = "uninstall",
            alias = "remove",
            about = "Uninstall repositories.",
            display_order = 5
        )]
        Uninstall(RepositoryUninstallArguments),

        #[command(
            name = "downgrade",
            about = "Downgrade a repositories.",
            display_order = 5
        )]
        Downgrade(RepositoryDowngradeArguments),
    }

    #[derive(Debug, Args)]
    pub struct PackageListArguments {
        #[arg(
            long = "exclude",
            alias = "ignore",
            short = 'e',
            help = "Partial or full name(s) of packages to exclude from the search among the installed packages.",
            display_order = 1
        )]
        pub excluded_partial_package_names: Vec<String>,

        #[arg(
            help = "Partial or full name(s) of the packages to search among the installed packages. Not specifying this argument will list all packages.",
            display_order = usize::MAX - 1,
        )]
        pub partial_package_name: Vec<String>,
    }

    #[derive(Debug, Args)]
    pub struct PackageSearchArguments {
        #[arg(
            long = "exclude",
            alias = "ignore",
            short = 'e',
            help = "Partial or full name(s) of packages to exclude from the search among available packages.",
            display_order = 1
        )]
        pub excluded_partial_package_names: Vec<String>,

        #[arg(
            help = "Partial or full name(s) of the packages to search among available packages.",
            last = true,
            display_order = usize::MAX - 1
        )]
        pub partial_package_name: Vec<String>,
    }

    #[derive(Debug, Args)]
    pub struct PackageInstallArguments {
        #[arg(help = "Full name(s) of the packages to install.", display_order = usize::MAX - 1)]
        pub package_names: Vec<String>,
    }

    #[derive(Debug, Args)]
    pub struct PackageUpdateArguments {
        #[arg(
            long = "exclude",
            alias = "ignore",
            short = 'e',
            help = "Full name(s) of packages to exclude from updating.",
            display_order = 1
        )]
        pub excluded_package_names: Vec<String>,

        #[arg(
            help = "Full name(s) of the packages to update. Not specifying this argument will update all packages",
            last = true,
            display_order = usize::MAX - 1
        )]
        pub package_names: Vec<String>,
    }

    #[derive(Debug, Args)]
    pub struct PackageUninstallArguments {
        #[arg(
            help = "Full name(s) of the packages to uninstall.",
            last = true,
            display_order = usize::MAX - 1
        )]
        pub package_names: Vec<String>,
    }

    #[derive(Debug, Args)]
    pub struct PackageDowngradeArguments {
        #[arg(
            long = "version",
            alias = "ver",
            help = "The version to downgrade to.",
            display_order = 1
        )]
        pub version: Option<String>,

        #[arg(
            help = "Full name of the package to downgrade.",
            last = true,
            display_order = usize::MAX - 1
        )]
        pub package_name: String,
    }

    #[derive(Debug, Args)]
    pub struct RepositoryListArguments {
        #[arg(
            long = "exclude",
            alias = "ignore",
            short = 'e',
            help = "Partial or full name(s) of repositories to exclude from the search among the installed repositories.",
            display_order = 1
        )]
        pub excluded_partial_repository_names: Vec<String>,

        #[arg(
            help = "Partial or full name(s) of the repositories to search among the installed repositories. Not specifying this argument will list all repositories.",
            last = true,
            display_order = usize::MAX - 1,
        )]
        pub partial_repository_name: Vec<String>,
    }

    #[derive(Debug, Args)]
    pub struct RepositorySearchArguments {
        #[arg(
            long = "exclude",
            alias = "ignore",
            short = 'e',
            help = "Partial or full name(s) of repositories to exclude from the search among available repositories.",
            display_order = 1
        )]
        pub excluded_partial_repository_names: Vec<String>,

        #[arg(
            help = "Partial or full name(s) of the repositories to search among available repositories.",
            last = true,
            display_order = usize::MAX - 1
        )]
        pub partial_repository_name: String,
    }

    #[derive(Debug, Args)]
    pub struct RepositoryInstallArguments {
        #[arg(help = "Full name(s) of the repositories to install.", display_order = usize::MAX - 1)]
        pub repository_names: Vec<String>,
    }

    #[derive(Debug, Args)]
    pub struct RepositoryUpdateArguments {
        #[arg(
            long = "exclude",
            alias = "ignore",
            short = 'e',
            help = "Full name(s) of repositories to exclude from updating.",
            display_order = 1
        )]
        pub excluded_repository_names: Vec<String>,

        #[arg(
            help = "Full name(s) of the repositories to update. Not specifying this argument will update all repositories",
            last = true,
            display_order = usize::MAX - 1
        )]
        pub repository_names: Vec<String>,
    }

    #[derive(Debug, Args)]
    pub struct RepositoryUninstallArguments {
        #[arg(
            help = "Full name(s) of the repositories to uninstall.",
            last = true,
            display_order = usize::MAX - 1
        )]
        pub repository_names: Vec<String>,
    }

    #[derive(Debug, Args)]
    pub struct RepositoryDowngradeArguments {
        #[arg(
            long = "version",
            alias = "ver",
            help = "The version to downgrade to.",
            display_order = 1
        )]
        pub version: Option<String>,

        #[arg(
            help = "Full name of the repository to downgrade.",
            last = true,
            display_order = usize::MAX - 1
        )]
        pub repository_name: String,
    }

    // region: IMPORTS

    use std::path::PathBuf;

    use clap::{Args, Parser, Subcommand};
    use paxy::ui;

    // endregion: IMPORTS
}

// endregion: MODULES

// region: RE-EXPORTS

pub use cli_template::*;

// endregion: RE-EXPORTS
