use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
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
