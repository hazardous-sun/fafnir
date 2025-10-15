// src/main.rs
mod cli;
mod commands;

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
    }

    Ok(())
}
