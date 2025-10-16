use crate::cli::CollectArgs;
use crate::utils::logger;
use anyhow::{Context, Result};
use ignore::{overrides::OverrideBuilder, DirEntry, WalkBuilder};
use serde_json::{Map, Value};
use std::fs;
use std::path::Path;

pub fn run(args: &CollectArgs) -> Result<()> {
    // 1. Set up the directory walker using the provided arguments
    let mut walk_builder = WalkBuilder::new(&args.path);

    // Use an OverrideBuilder to add programmatic ignore patterns
    let mut override_builder = OverrideBuilder::new(&args.path);

    // Add patterns from --ignore-all
    for pattern in &args.ignore_all {
        // '!' makes it an ignore rule, not a whitelist
        override_builder.add(&format!("!**/{}", pattern))
            .context(format!("Failed to add ignore-all pattern: {}", pattern))?;
    }

    // Add patterns from --ignore
    for pattern in &args.ignore {
        override_builder.add(&format!("!{}", pattern))
            .context(format!("Failed to add ignore pattern: {}", pattern))?;
    }

    // Always ignore the output file itself
    if let Some(output_filename) = args.output_file.to_str() {
        override_builder.add(&format!("!{}", output_filename))
            .context(format!("Failed to ignore output file: {}", output_filename))?;
    }

    // Build the override rules and apply them to the WalkBuilder
    let overrides = override_builder.build()?;
    walk_builder.overrides(overrides);

    // 2. Walk the directory and build the JSON structure
    let mut root_map = Map::new();

    for result in walk_builder.build() {
        let entry = result.context("Failed to process a directory entry")?;
        if entry.file_type().map_or(false, |ft| ft.is_file()) {
            insert_file_content(&mut root_map, &entry, &args.path)?;
        }
    }

    let final_json = Value::Object(root_map);

    // 3. Write the JSON to the output file
    let file = fs::File::create(&args.output_file)
        .with_context(|| format!("Failed to create output file: {:?}", &args.output_file))?;

    serde_json::to_writer_pretty(file, &final_json)
        .context("Failed to write JSON to output file")?;

    logger::info(&format!(
        "Repository content successfully saved to {:?}",
        &args.output_file
    ));
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
