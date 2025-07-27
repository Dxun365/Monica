use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, author, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    commands: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {}

pub fn start() {
    let cli = Cli::parse();

    println!("the cli of value : {:?}", cli);
}
