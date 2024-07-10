use clap::Parser;
use color_eyre::Result;
use env_logger::Env;

mod command;
mod run_script;
use command::{Cli, Command};

#[tokio::main]
async fn main() -> Result<()> {
    let env = Env::new()
        .filter("DINOPKG_LOG")
        .write_style("DINOPKG_LOG_STYLE");
    color_eyre::install()?;
    env_logger::try_init_from_env(env)?;

    let cli = Cli::parse();
    match cli.command {
        Command::Run { script_name } => command::run::run(script_name).await?,
    }
    Ok(())
}
