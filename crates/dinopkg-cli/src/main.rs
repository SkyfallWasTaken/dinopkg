use clap::Parser;

mod command;
use color_eyre::Result;
use command::{Cli, Command};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Run { script_name } => command::run::run(script_name).await?,
    }
    Ok(())
}
