use clap::Parser;

mod command;
use command::{Cli, Command};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    cli.command.run().await;
    println!("Hello, world!");
}
