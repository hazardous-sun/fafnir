use clap::Parser;
use std::path::PathBuf;

/// A structure to collect the parameters required for running the collector utility.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
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
