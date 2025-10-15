use anyhow::{Context, Result};
use clap::Parser;
use ignore::{DirEntry, WalkBuilder};
use serde_json::{Map, Value};
use std::fs;
use std::path::{Path, PathBuf};

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

pub fn run() -> Result<()> {
    let config = Cli::parse();

    // 1. Set up the directory walker with custom ignore rules
    let mut walk_builder = WalkBuilder::new(&config.root);
    walk_builder.add_custom_ignore_filename(".gitignore");

    // Add patterns from --ignore-all
    for pattern in &config.ignore_all {
        walk_builder.add_ignore(format!("**/{}", pattern));
    }

    // Add patterns from --ignore
    for pattern in &config.ignore {
        walk_builder.add_ignore(pattern);
    }

    // Always ignore the output file itself
    if let Some(output_filename) = config.output_file.to_str() {
        walk_builder.add_ignore(output_filename);
    }

    // 2. Walk the directory and build the JSON structure
    let mut root_map = Map::new();

    for result in walk_builder.build() {
        let entry = result.context("Failed to process a directory entry")?;
        if entry.file_type().map_or(false, |ft| ft.is_file()) {
            insert_file_content(&mut root_map, &entry, &config.root)?;
        }
    }

    let final_json = Value::Object(root_map);

    // 3. Write the JSON to the output file
    let file = fs::File::create(&config.output_file)
        .with_context(|| format!("Failed to create output file: {:?}", &config.output_file))?;

    serde_json::to_writer_pretty(file, &final_json)
        .context("Failed to write JSON to output file")?;

    println!(
        "✅ Repository content successfully saved to {:?}",
        &config.output_file
    );
    Ok(())
}

fn insert_file_content(
    map: &mut Map<String, Value>,
    entry: &DirEntry,
    root_path: &Path,
) -> Result<()> {
    let path = entry
        .path()
        .strip_prefix(root_path)
        .unwrap_or_else(|_| entry.path());
    let mut current_map = map;
    let components: Vec<_> = path.components().collect();

    if let Some((filename_component, parent_components)) = components.split_last() {
        // Create nested directories
        for component in parent_components {
            let dir_name = component.as_os_str().to_string_lossy().to_string();
            let entry = current_map
                .entry(dir_name)
                .or_insert_with(|| Value::Object(Map::new()));
            current_map = entry.as_object_mut().unwrap();
        }

        // Insert file content
        let filename = filename_component.as_os_str().to_string_lossy().to_string();

        // Read file content
        let content = fs::read_to_string(entry.path())
            .unwrap_or_else(|_| "[Error: Non-UTF8 or unreadable file]".to_string());

        current_map.insert(filename, Value::String(content));
    }

    Ok(())
}
