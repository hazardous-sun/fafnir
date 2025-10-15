mod cli;
mod commands;
mod utils;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Match the subcommand and call its run function
    match &cli.command {
        Commands::Collect(args) => {
            commands::collect::run(args)?;
        }
        Commands::CheckRepos(args) => {
            commands::check_repos::run(args)?;
        }
    }

    Ok(())
}
