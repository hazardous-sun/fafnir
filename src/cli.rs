// src/cli.rs
use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// The top-level CLI structure.
#[derive(Parser, Debug)]
#[command(name = "collector", version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// An enum to hold all possible subcommands. For now, it's just 'collect'.
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Collects repository content into a single JSON file.
    Collect(CollectArgs),
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
