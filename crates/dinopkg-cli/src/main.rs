use clap::Parser;
use env_logger::{Builder, Env};

mod command;
use color_eyre::Result;
use command::{Cli, Command};

#[tokio::main]
async fn main() -> Result<()> {
    let env = Env::new()
        .filter("DINOPKG_LOG")
        .write_style("DINOPKG_LOG_STYLE");
    env_logger::try_init_from_env(env)?;

    let cli = Cli::parse();
    match cli.command {
        Command::Run { script_name } => command::run::run(script_name).await?,
    }
    Ok(())
}
