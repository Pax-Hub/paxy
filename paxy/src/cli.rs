//! The commandline interface of the program with which users interact on a
//! text-based console.

pub use cli_template::*; // Flatten the cli_template module so that users do
                         // not need to handle it. It is for our convenience

use clap::Parser;

pub trait CommandlineFlags {
    fn json_flag(&self) -> bool;
    fn plain_flag(&self) -> bool;
    fn debug_flag(&self) -> bool;
    fn output_kind(&self) -> CommandlineOutputKind {
        if self.json_flag() {
            return CommandlineOutputKind::Json;
        } else if self.plain_flag() {
            return CommandlineOutputKind::Plain;
        } else if self.debug_flag() {
            return CommandlineOutputKind::Debug;
        } else {
            return CommandlineOutputKind::Regular;
        }
    }
}

#[derive(Debug)]
pub enum CommandlineOutputKind {
    Regular,
    Json,
    Plain,
    Debug,
}

pub struct CommandlineDispatcher {}

impl CommandlineDispatcher {
    pub fn new() -> Self {
        CommandlineDispatcher {}
    }

    pub fn run(self) {
        let cli = Cli::parse();

        println!(
            "\nThe commandline argument structure is as below: \n{:#?}",
            cli
        );

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

    use super::CommandlineFlags;
    use clap::{Args, Parser, Subcommand};
    use std::path::PathBuf;

    /// The big picture of the commandline-interface. This structure contains
    /// the outermost sub-commands of the interface
    #[derive(Debug, Parser, PartialEq)]
    #[command(version, author, about, args_conflicts_with_subcommands = true)]
    pub struct Cli {
        /// Global arguments like `--json`, `--plain`, and `--debug`
        #[clap(flatten)]
        pub global_arguments: GlobalArguments,

        /// Any subcommand variant to perform an action with the program
        #[clap(subcommand)]
        pub command: Option<ActionCommand>,

        /// When a subcommand is absent, update by default
        #[clap(flatten)]
        pub arguments: UpdateActionArguments,
    }

    /// The arguments that are available regardless of the subcommand used
    #[derive(Debug, Args, PartialEq)]
    #[clap(args_conflicts_with_subcommands = true)]
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
            return self.json_flag;
        }

        fn plain_flag(&self) -> bool {
            return self.plain_flag;
        }

        fn debug_flag(&self) -> bool {
            return self.debug_flag;
        }
    }

    /// A subcommand variant to perform an action with the program
    #[derive(Debug, Subcommand, PartialEq)]
    #[clap(args_conflicts_with_subcommands = true)]
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

    #[derive(Debug, Args, PartialEq)]
    pub struct ListActionArguments {
        #[clap(
            help = "Partial or full name(s) of the package(s) to search for, among the installed packages.",
            num_args = 0..,
            display_order = 19
        )]
        pub package_names: Vec<String>, // This should always be the last argument
    }

    #[derive(Debug, Args, PartialEq)]
    pub struct SearchActionArguments {
        #[clap(
            help = "Partial or full name(s) of the package(s) to list, among available packages.",
            num_args = 0..,
            display_order = 19
        )]
        pub package_names: Vec<String>, // This should always be the last argument
    }

    #[derive(Debug, Args, PartialEq)]
    pub struct InstallActionArguments {
        #[clap(
            help = "Partial or full name(s) of the package(s) to *install*, among available packages.",
            num_args = 0..,
            display_order = 19
        )]
        pub package_names: Vec<String>, // This should always be the last argument
    }

    #[derive(Debug, Args, PartialEq)]
    pub struct UpdateActionArguments {
        #[clap(
            help = "Partial or full name(s) of the package(s) to *update*, among the installed packages.",
            num_args = 0..,
            display_order = 19
        )]
        pub package_names: Vec<String>, // This should always be the last argument
    }

    #[derive(Debug, Args, PartialEq)]
    pub struct RemoveActionArguments {
        #[clap(
            help = "Partial or full name(s) of the package(s) to *remove*, among the installed packages.",
            num_args = 0..,
            display_order = 19
        )]
        pub package_names: Vec<String>, // This should always be the last argument
    }

    #[derive(Debug, Args, PartialEq)]
    pub struct EnvironmentActionArguments {
        #[clap(
            help = "Partial or full name(s) of the package(s) to create an environment from, among the available packages.",
            num_args = 0..,
            display_order = 19
        )]
        pub package_names: Vec<String>, // This should always be the last argument
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn list() {
            let cli: Cli = Parser::parse_from(["paxy", "list", "foo", "bar", "baz"]);
            assert_eq!(
                cli,
                Cli {
                    global_arguments: GlobalArguments {
                        json_flag: false,
                        plain_flag: false,
                        debug_flag: false,
                        configuration_file: None
                    },
                    command: Some(ActionCommand::List(ListActionArguments {
                        package_names: vec![
                            "foo".to_string(),
                            "bar".to_string(),
                            "baz".to_string(),
                        ]
                    })),
                    arguments: UpdateActionArguments {
                        package_names: vec![],
                    }
                }
            );
        }

        #[test]
        fn search() {
            let cli: Cli = Parser::parse_from(["paxy", "search", "foo", "bar", "baz"]);
            assert_eq!(
                cli,
                Cli {
                    global_arguments: GlobalArguments {
                        json_flag: false,
                        plain_flag: false,
                        debug_flag: false,
                        configuration_file: None
                    },
                    command: Some(ActionCommand::Search(SearchActionArguments {
                        package_names: vec![
                            "foo".to_string(),
                            "bar".to_string(),
                            "baz".to_string(),
                        ]
                    })),
                    arguments: UpdateActionArguments {
                        package_names: vec![],
                    }
                }
            );
        }

        #[test]
        fn install() {
            let cli: Cli = Parser::parse_from(["paxy", "install", "foo", "bar", "baz"]);
            assert_eq!(
                cli,
                Cli {
                    global_arguments: GlobalArguments {
                        json_flag: false,
                        plain_flag: false,
                        debug_flag: false,
                        configuration_file: None
                    },
                    command: Some(ActionCommand::Install(InstallActionArguments {
                        package_names: vec![
                            "foo".to_string(),
                            "bar".to_string(),
                            "baz".to_string(),
                        ]
                    })),
                    arguments: UpdateActionArguments {
                        package_names: vec![],
                    }
                }
            );
        }

        #[test]
        fn update() {
            let cli: Cli = Parser::parse_from(["paxy", "update", "foo", "bar", "baz"]);
            assert_eq!(
                cli,
                Cli {
                    global_arguments: GlobalArguments {
                        json_flag: false,
                        plain_flag: false,
                        debug_flag: false,
                        configuration_file: None
                    },
                    command: Some(ActionCommand::Update(UpdateActionArguments {
                        package_names: vec![
                            "foo".to_string(),
                            "bar".to_string(),
                            "baz".to_string(),
                        ]
                    })),
                    arguments: UpdateActionArguments {
                        package_names: vec![],
                    }
                }
            );
        }

        #[test]
        fn remove() {
            let cli: Cli = Parser::parse_from(["paxy", "remove", "foo", "bar", "baz"]);
            assert_eq!(
                cli,
                Cli {
                    global_arguments: GlobalArguments {
                        json_flag: false,
                        plain_flag: false,
                        debug_flag: false,
                        configuration_file: None
                    },
                    command: Some(ActionCommand::Remove(RemoveActionArguments {
                        package_names: vec![
                            "foo".to_string(),
                            "bar".to_string(),
                            "baz".to_string(),
                        ]
                    })),
                    arguments: UpdateActionArguments {
                        package_names: vec![],
                    }
                }
            );
        }

        #[test]
        fn environment() {
            let cli: Cli = Parser::parse_from(["paxy", "env", "foo", "bar", "baz"]);
            assert_eq!(
                cli,
                Cli {
                    global_arguments: GlobalArguments {
                        json_flag: false,
                        plain_flag: false,
                        debug_flag: false,
                        configuration_file: None
                    },
                    command: Some(ActionCommand::Environment(EnvironmentActionArguments {
                        package_names: vec![
                            "foo".to_string(),
                            "bar".to_string(),
                            "baz".to_string(),
                        ]
                    })),
                    arguments: UpdateActionArguments {
                        package_names: vec![],
                    }
                }
            );
        }
    }
}
