use clap::{Parser, Subcommand};

pub mod run;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run a script in package.json
    Run {
        /// The name of the script to run
        script_name: Option<String>,
    },
}

pub(crate) trait Command {
    async fn run(command: &Commands);
}

impl Command for Commands {
    async fn run(&self) {
        match self {
            Commands::Run { .. } => {
                run::Run::run(self).await;
            }
        }
    }
}
