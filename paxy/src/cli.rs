#![warn(missing_docs)]

//! A module to group the code that handles the commandline interface of `paxy`.

pub use cli_template::*; // Flatten the `cli_template` module so that we do
                         // not need to explicitly include t. It is strictly for
                         // grouping convenience.

use clap::Parser;

/// A trait that infers the type of display desired when boolean flags are
/// specified.
trait CommandlineFlags {
    fn json_flag(&self) -> bool;
    fn plain_flag(&self) -> bool;
    fn debug_flag(&self) -> bool;
    fn output_kind(&self) -> CommandlineOutputKind {
        CommandlineOutputKind::from_flags(self.json_flag(), self.plain_flag(), self.debug_flag())
    }
}

/// An enumeration that represents a user's choice for how to display output.
#[derive(Debug)]
pub enum CommandlineOutputKind {
    Regular,
    Json,
    Plain,
    Debug,
}

impl CommandlineOutputKind {
    /// Returns an enumeration representing how to display the output, based on
    /// user-specified boolean flags for *json*, *plain* or *debug* output.
    fn from_flags(json_flag: bool, plain_flag: bool, debug_flag: bool) -> Self {
        if json_flag {
            CommandlineOutputKind::Json
        } else if plain_flag {
            CommandlineOutputKind::Plain
        } else if debug_flag {
            CommandlineOutputKind::Debug
        } else {
            CommandlineOutputKind::Regular
        }
    }
}

/// A structure that takes in user input from the commandline and dispatches
/// that input data to the appropriate code for performing an inferred task.
pub struct CommandlineDispatcher {}

impl CommandlineDispatcher {
    /// Parses commandline arguments accordinng to the `clap::_derive` template
    /// specified by [`Cli`]
    pub fn run() {
        let cli = Cli::parse();

        println!("\nThe commandline argument structure is as below: \n{cli:#?}",);

        println!(
            "\nThe kind of output desired is: \n{:#?}\n",
            cli.global_arguments.output_kind()
        );

        // match cli.command {
        //     Some(ActionCommand::List(list_action_arguments)) => {
        //         // Use list_action_arguments.package_names
        //     }
        //     Some(ActionCommand::Search(search_action_arguments)) => {
        //         // Use search_action_arguments.package_names
        //     }
        //     Some(ActionCommand::Install(install_action_arguments)) => {
        //         // Use install_action_arguments.package_names
        //     }
        //     Some(ActionCommand::Update(update_action_arguments)) => {
        //         // Use update_action_arguments.package_names
        //     }
        //     Some(ActionCommand::Remove(remove_action_arguments)) => {
        //         // Use remove_action_arguments.package_names
        //     }
        //     Some(ActionCommand::Environment(environment_action_arguments)) => {
        //         // Use environment_action_arguments.package_names
        //     }
        //     None => {
        //         // Use cli.arguments.package_names
        //     }
        // }
    }
}

/// A module to group together all the data structures that make up the
/// `derive` interface of the `clap` library. This is separate from any of the
/// helper functions that are used as part of the commandline interface.
pub mod cli_template {

    #![warn(missing_docs)]

    use super::CommandlineFlags;
    use clap::{Args, Parser, Subcommand};
    use std::path::PathBuf;

    /// The big picture of the commandline-interface. This structure contains
    /// the outermost sub-commands of the interface.
    #[derive(Debug, Parser, PartialEq)]
    #[clap(author, version, about)]
    pub struct Cli {
        /// Global arguments like `--json`, `--plain`, and `--debug`
        #[clap(flatten)]
        pub global_arguments: GlobalArguments,

        /// Any subcommand variant to perform an action with the program.
        #[clap(subcommand)]
        pub command: ActionCommand,
    }

    /// The arguments that are available regardless of the subcommand used.
    #[derive(Debug, Args, PartialEq)]
    pub struct GlobalArguments {
        #[clap(
                long = "json",
                help = "Output in the JSON format for machine readability and scripting purposes.",
                num_args = 0,
                global = true,
                display_order = usize::MAX - 1,
            )]
        pub json_flag: bool,

        #[clap(
                long = "plain",
                help = "Output as plain text without extra information, for machine readability and scripting purposes.",
                num_args = 0,
                global = true,
                display_order = usize::MAX - 2,
            )]
        pub plain_flag: bool,

        #[clap(
                long = "debug",
                help = "Output debug messages.",
                num_args = 0,
                global = true,
                display_order = usize::MAX - 3,
            )]
        pub debug_flag: bool,

        #[clap(
            long = "conf",
            help = "Specify a configuration file to use.",
            num_args = 1,
            global = true,
            display_order = usize::MAX - 4,
        )]
        pub configuration_file: Option<PathBuf>,
    }

    impl CommandlineFlags for GlobalArguments {
        fn json_flag(&self) -> bool {
            self.json_flag
        }

        fn plain_flag(&self) -> bool {
            self.plain_flag
        }

        fn debug_flag(&self) -> bool {
            self.debug_flag
        }
    }

    /// A subcommand variant to perform an action with the program.
    #[derive(Debug, Subcommand, PartialEq)]
    pub enum ActionCommand {
        #[clap(name = "list", about = "List installed package(s).", display_order = 1)]
        List(ListActionArguments),

        #[clap(
            name = "search",
            about = "Search for available package(s).",
            display_order = 2
        )]
        Search(SearchActionArguments),

        #[clap(
            name = "install",
            alias = "add",
            about = "Install Package(s).",
            display_order = 3
        )]
        Install(InstallActionArguments),

        #[clap(
            name = "update",
            alias = "upgrade",
            about = "Update Package(s).",
            display_order = 4
        )]
        Update(UpdateActionArguments),

        #[clap(
            name = "remove",
            alias = "uninstall",
            about = "Remove Package(s).",
            display_order = 5
        )]
        Remove(RemoveActionArguments),

        #[clap(
            name = "env",
            alias = "environment",
            about = "Install Package(s) in a sandboxed container and create a shell.",
            display_order = 6
        )]
        Environment(EnvironmentActionArguments),
    }

    /// Arguments for the `list` subcommand.
    #[derive(Debug, Args, PartialEq)]
    pub struct ListActionArguments {
        #[clap(
            help = "Partial or full name(s) of the package(s) to search for, among the installed packages.",
            num_args = 0..,
            display_order = 19
        )]
        pub package_names: Vec<String>, // This should always be the last argument
    }

    /// Arguments for the `search` subcommand.
    #[derive(Debug, Args, PartialEq)]
    pub struct SearchActionArguments {
        #[clap(
            help = "Partial or full name(s) of the package(s) to list, among available packages.",
            num_args = 1..,
            required = true,
            display_order = 29
        )]
        pub package_names: Vec<String>, // This should always be the last argument
    }

    /// Arguments for the `install` subcommand.
    #[derive(Debug, Args, PartialEq)]
    pub struct InstallActionArguments {
        #[clap(
            help = "Partial or full name(s) of the package(s) to *install*, among available packages.",
            num_args = 1..,
            required = true,
            display_order = 39
        )]
        pub package_names: Vec<String>, // This should always be the last argument
    }

    /// Arguments for the `update` subcommand.
    #[derive(Debug, Args, PartialEq)]
    pub struct UpdateActionArguments {
        #[clap(
            help = "Partial or full name(s) of the package(s) to *update*, among the installed packages.",
            num_args = 0..,
            display_order = 49
        )]
        pub package_names: Vec<String>, // This should always be the last argument
    }

    /// Arguments for the `remove` subcommand.
    #[derive(Debug, Args, PartialEq)]
    pub struct RemoveActionArguments {
        #[clap(
            help = "Partial or full name(s) of the package(s) to *remove*, among the installed packages.",
            num_args = 1..,
            required = true,
            display_order = 59
        )]
        pub package_names: Vec<String>, // This should always be the last argument
    }

    /// Arguments for the `env` subcommand.
    #[derive(Debug, Args, PartialEq)]
    pub struct EnvironmentActionArguments {
        #[clap(
            help = "Partial or full name(s) of the package(s) to create an environment from, among the available packages.",
            num_args = 1..,
            required = true,
            display_order = 69
        )]
        pub package_names: Vec<String>, // This should always be the last argument
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn list() {
            let cli = Cli::parse_from(["paxy", "list", "foo", "bar", "baz"]);
            assert_eq!(
                cli,
                Cli {
                    global_arguments: GlobalArguments {
                        json_flag: false,
                        plain_flag: false,
                        debug_flag: false,
                        configuration_file: None
                    },
                    command: ActionCommand::List(ListActionArguments {
                        package_names: vec![
                            "foo".to_string(),
                            "bar".to_string(),
                            "baz".to_string(),
                        ]
                    }),
                }
            );
        }

        #[test]
        fn search() {
            let cli = Cli::parse_from(["paxy", "search", "foo", "bar", "baz"]);
            assert_eq!(
                cli,
                Cli {
                    global_arguments: GlobalArguments {
                        json_flag: false,
                        plain_flag: false,
                        debug_flag: false,
                        configuration_file: None
                    },
                    command: ActionCommand::Search(SearchActionArguments {
                        package_names: vec![
                            "foo".to_string(),
                            "bar".to_string(),
                            "baz".to_string(),
                        ]
                    }),
                }
            );
        }

        #[test]
        fn install() {
            let cli = Cli::parse_from(["paxy", "install", "foo", "bar", "baz"]);
            assert_eq!(
                cli,
                Cli {
                    global_arguments: GlobalArguments {
                        json_flag: false,
                        plain_flag: false,
                        debug_flag: false,
                        configuration_file: None
                    },
                    command: ActionCommand::Install(InstallActionArguments {
                        package_names: vec![
                            "foo".to_string(),
                            "bar".to_string(),
                            "baz".to_string(),
                        ]
                    }),
                }
            );
        }

        #[test]
        fn update() {
            let cli = Cli::parse_from(["paxy", "update", "foo", "bar", "baz"]);
            assert_eq!(
                cli,
                Cli {
                    global_arguments: GlobalArguments {
                        json_flag: false,
                        plain_flag: false,
                        debug_flag: false,
                        configuration_file: None
                    },
                    command: ActionCommand::Update(UpdateActionArguments {
                        package_names: vec![
                            "foo".to_string(),
                            "bar".to_string(),
                            "baz".to_string(),
                        ]
                    }),
                }
            );
        }

        #[test]
        fn remove() {
            let cli = Cli::parse_from(["paxy", "remove", "foo", "bar", "baz"]);
            assert_eq!(
                cli,
                Cli {
                    global_arguments: GlobalArguments {
                        json_flag: false,
                        plain_flag: false,
                        debug_flag: false,
                        configuration_file: None
                    },
                    command: ActionCommand::Remove(RemoveActionArguments {
                        package_names: vec![
                            "foo".to_string(),
                            "bar".to_string(),
                            "baz".to_string(),
                        ]
                    }),
                }
            );
        }

        #[test]
        fn environment() {
            let cli = Cli::parse_from(["paxy", "env", "foo", "bar", "baz"]);
            assert_eq!(
                cli,
                Cli {
                    global_arguments: GlobalArguments {
                        json_flag: false,
                        plain_flag: false,
                        debug_flag: false,
                        configuration_file: None
                    },
                    command: ActionCommand::Environment(EnvironmentActionArguments {
                        package_names: vec![
                            "foo".to_string(),
                            "bar".to_string(),
                            "baz".to_string(),
                        ]
                    }),
                }
            );
        }

        #[test]
        fn list_empty() {
            let cli = Cli::parse_from(["paxy", "list"]);
            assert_eq!(
                cli,
                Cli {
                    global_arguments: GlobalArguments {
                        json_flag: false,
                        plain_flag: false,
                        debug_flag: false,
                        configuration_file: None
                    },
                    command: ActionCommand::List(ListActionArguments {
                        package_names: vec![],
                    }),
                }
            );
        }

        #[test]
        fn search_empty() {
            let cli = Cli::try_parse_from(["paxy", "search"]);
            assert!(cli.is_err());
        }

        #[test]
        fn install_empty() {
            let cli = Cli::try_parse_from(["paxy", "install"]);
            assert!(cli.is_err())
        }

        #[test]
        fn update_empty() {
            let cli = Cli::parse_from(["paxy", "update"]);
            assert_eq!(
                cli,
                Cli {
                    global_arguments: GlobalArguments {
                        json_flag: false,
                        plain_flag: false,
                        debug_flag: false,
                        configuration_file: None
                    },
                    command: ActionCommand::Update(UpdateActionArguments {
                        package_names: vec![]
                    }),
                }
            );
        }

        #[test]
        fn remove_empty() {
            let cli = Cli::try_parse_from(["paxy", "remove"]);
            assert!(cli.is_err());
        }

        #[test]
        fn environment_empty() {
            let cli = Cli::try_parse_from(["paxy", "env"]);
            assert!(cli.is_err());
        }
    }
}
