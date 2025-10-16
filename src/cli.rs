use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// A collection of handy command-line tools for developers.
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
    CheckRepos(RemoteOperationArgs),
    /// Pulls the latest's changes in the current branch for all git repositories
    /// in one or more directories.
    PullRepos(RemoteOperationArgs),
    /// Pushes the latest's changes in the current branch for all git repositores
    /// in one or more directories.
    PushRepos(RemoteOperationArgs),
}

/// Arguments specific to the 'collect' command.
#[derive(Parser, Debug)]
pub struct CollectArgs {
    /// The root directory to start scanning from. Defaults to the CWD.
    #[arg(long, short = 'p', default_value = ".")]
    pub path: PathBuf,

    /// The path to the output file. Defaults to `content.json`.
    #[arg(long, short = 'o', default_value = "content.json")]
    pub output_file: PathBuf,

    /// Specific file or directory paths to ignore.
    #[arg(long, short = 'i', value_name = "PATH", num_args = 1..)]
    pub ignore: Vec<String>,

    /// File or directory names to ignore globally, regardless of their path.
    #[arg(long, value_name = "FILENAME", num_args = 1..)]
    pub ignore_all: Vec<String>,
}

/// Arguments specific to the `check-repos`, `pull-repos`, and `push-repos` commands.
#[derive(Parser, Debug)]
pub struct RemoteOperationArgs {
    /// The parent directories to run the remote operations.
    #[arg(required = true, num_args = 1..)]
    pub directories: Vec<PathBuf>,
}
