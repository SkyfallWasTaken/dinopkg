use clap::Parser;

mod command;
use command::Cli;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    println!("Hello, world!");
}
