use clap::Parser;

mod command;
use command::{Cli, Command};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Run { script_name } => command::run::run(script_name).await,
    }
}
