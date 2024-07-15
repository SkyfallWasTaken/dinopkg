use clap::{Parser, Subcommand};

pub mod init;
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
    #[command(aliases = ["rum", "urn", "run-script"])]
    Run {
        /// The name of the script to run
        script_name: Option<String>,
    },

    /// Run tests for a package
    #[command(aliases = ["tst", "t"])]
    Test,

    /// Create a package.json file
    #[command(aliases = ["create", "innit"])]
    Init,
}
