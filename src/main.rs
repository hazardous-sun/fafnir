mod cli;
mod commands;
mod utils;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    utils::logger::init_verbose(cli.verbose);

    // Match the subcommand and call its run function
    match &cli.command {
        Commands::Collect(args) => {
            commands::git_utilities::collect::run(args)?;
        }
        Commands::CheckRepos(args) => {
            commands::git_utilities::check_repos::run(args)?;
        }
        Commands::PullRepos(args) => {
            commands::git_utilities::pull_repos::run(args)?;
        }
        Commands::PushRepos(args) => {
            commands::git_utilities::push_repos::run(args)?;
        }
    }

    Ok(())
}
