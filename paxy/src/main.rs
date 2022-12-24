use clap::Parser;

mod cli;

fn main() {
    println!("{:?}", cli::Args::parse());
}
