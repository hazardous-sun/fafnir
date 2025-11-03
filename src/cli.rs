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

    /// Renames multiple files (and explicitly-passed directories) in bulk.
    ///
    /// Examples:
    ///   fafnir bulk-rename "*.java" "{}.md"
    ///   fafnir bulk-rename "*.feature" "test-file-{}.md"
    ///   fafnir bulk-rename teste1.md teste2.md teste3.md "{}.go"
    BulkRename(BulkRenameArgs),
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

/// Arguments for `bulk-rename`.
#[derive(Parser, Debug)]
pub struct BulkRenameArgs {
    /// One or more absolute/relative paths or glob patterns (e.g., *.java, src/*.md, /home/me/file*).
    /// When using glob patterns, only files are matched. Directories are only renamed if passed explicitly.
    #[arg(required = true, num_args = 1..)]
    pub patterns: Vec<String>,

    /// The expression used to build new names. Use '{}' for the original name *without* extension.
    /// Examples: "{}.md", "test-file-{}.md"
    #[arg(required = true)]
    pub replacement: String,

    /// Recursively search within directories inferred from the patterns.
    #[arg(short = 'r', long = "recursive", action = clap::ArgAction::SetTrue)]
    pub recursive: bool,
}
