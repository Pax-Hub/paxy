use clap::{Parser, Subcommand};

#[derive(Parser, Debug, PartialEq, Eq)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug, PartialEq, Eq)]
pub enum Command {
    #[command(name = "search", about = "Search for a package")]
    Search { query: Vec<String> },
    #[command(name = "install", about = "Install package(s)")]
    Install { packages: Vec<String> },
    #[command(name = "remove", about = "Uninstall package(s)")]
    Remove { packages: Vec<String> },
    #[command(name = "upgrade", about = "Upgrade package(s)")]
    Upgrade { packages: Vec<String> },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arg_parsing() {
        let args = Args::parse_from(["paxy", "search", "foo"]);
        assert_eq!(
            args,
            Args {
                command: Some(Command::Search {
                    query: vec!["foo".to_string()]
                })
            }
        );
    }
}
