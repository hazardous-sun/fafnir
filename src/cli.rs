use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// A structure to collect the CLI parameters and check which command should run.
#[derive(Parser, Debug)]
#[command(name = "collector", version, about, long_about = None)]
pub struct Cli {
    /// The command to run.
    #[command(subcommand)]
    pub command: Commands,

    /// Enables verbose output (debug logs).
    #[arg(long, global = true, action = clap::ArgAction::SetTrue)]
    pub verbose: bool,
}

/// An enum to hold all possible commands.
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Collects repository content into a single JSON file.
    Collect(CollectArgs),
    /// Checks the status of all git repositories in one or more directories.
    CheckRepos(CheckReposArgs),
}

/// Arguments specific to the 'collect' command.
#[derive(Parser, Debug)]
pub struct CollectArgs {
    /// The path to the output file. Defaults to `content.json`.
    #[arg(default_value = "content.json")]
    pub output_file: PathBuf,

    /// Specific file or directory paths to ignore.
    #[arg(long, short = 'i', value_name = "PATH")]
    pub ignore: Vec<String>,

    /// File or directory names to ignore globally, regardless of their path.
    #[arg(long, value_name = "FILENAME")]
    pub ignore_all: Vec<String>,

    /// The root directory to start scanning from. Defaults to the CWD.
    #[arg(default_value = ".")]
    pub root: PathBuf,
}

/// Arguments specific to the 'check-repos' command.
#[derive(Parser, Debug)]
pub struct CheckReposArgs {
    /// The parent directories to scan for git repositories.
    #[arg(required = true, num_args = 1..)]
    pub directories: Vec<PathBuf>,
}
