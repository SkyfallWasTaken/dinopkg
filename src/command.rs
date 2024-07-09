use clap::{Parser, Subcommand};

pub mod run;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Run a script in package.json
    Run {
        /// The name of the script to run
        script_name: Option<String>,
    },
}
