use crate::cli::BulkRenameArgs;
use crate::utils::logger;
use anyhow::{Context, Result};
use globset::{Glob, GlobMatcher};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn run(args: &BulkRenameArgs) -> Result<()> {
    // Build candidate list from patterns
    let mut candidates: Vec<Candidate> = Vec::new();

    for raw in &args.patterns {
        if is_glob_like(raw) {
            // Treat as a glob pattern: collect FILES only (never directories) to avoid Scenario 1 mistakes.
            let matcher = Glob::new(raw)
                .with_context(|| format!("Invalid glob pattern: {raw}"))?
                .compile_matcher();

            let search_root = infer_search_root(raw)?;
            if !search_root.exists() {
                logger::warning(&format!(
                    "Skipping '{}': inferred search root '{}' does not exist",
                    raw,
                    search_root.display()
                ));
                continue;
            }

            if args.recursive {
                for entry in WalkDir::new(&search_root).into_iter().filter_map(|e| e.ok()) {
                    if entry.file_type().is_file() && is_match(&matcher, raw, entry.path()) {
                        candidates.push(Candidate::glob(entry.into_path()));
                    }
                }
            } else {
                // Only search the directory level that was passed/inferred (non-recursive)
                if search_root.is_dir() {
                    for entry in fs::read_dir(&search_root)
                        .with_context(|| format!("Failed to read '{}'", search_root.display()))?
                    {
                        let entry = entry?;
                        let path = entry.path();
                        if path.is_file() && is_match(&matcher, raw, &path) {
                            candidates.push(Candidate::glob(path));
                        }
                    }
                } else if search_root.is_file() && is_match(&matcher, raw, &search_root) {
                    candidates.push(Candidate::glob(search_root.clone()));
                }
            }
        } else {
            // Treat as an explicit path: can be file OR directory. If it's a directory, we *will* rename it.
            let p = PathBuf::from(raw);
            if p.exists() {
                candidates.push(Candidate::explicit(p));
            } else {
                logger::warning(&format!("Path not found: {}", raw));
            }
        }
    }

    // Deduplicate candidates by their canonical (where possible) paths
    let mut seen: HashSet<PathBuf> = HashSet::new();
    candidates.retain(|c| {
        let key = canonical_or_self(&c.path);
        if seen.contains(&key) {
            false
        } else {
            seen.insert(key);
            true
        }
    });

    // Perform renames
    let mut renamed = 0usize;
    let mut skipped = 0usize;

    for c in candidates {
        // Skip directories that originated from a glob match (Scenario 1 safeguard).
        if c.kind == Source::Glob && c.path.is_dir() {
            logger::debug(&format!(
                "Skipping directory matched via glob: {}",
                c.path.display()
            ));
            skipped += 1;
            continue;
        }

        let new_path = compute_target_path(&c.path, &args.replacement)?;

        if new_path == c.path {
            logger::debug(&format!("Unchanged (same name): {}", c.path.display()));
            skipped += 1;
            continue;
        }

        if new_path.exists() {
            logger::error(&format!(
                "Target already exists, skipping: '{}' -> '{}'",
                c.path.display(),
                new_path.display()
            ));
            skipped += 1;
            continue;
        }

        fs::rename(&c.path, &new_path).with_context(|| {
            format!(
                "Failed to rename '{}' -> '{}'",
                c.path.display(),
                new_path.display()
            )
        })?;

        logger::info(&format!(
            "Renamed '{}' -> '{}'",
            c.path.display(),
            new_path.display()
        ));
        renamed += 1;
    }

    logger::info(&format!("Done. Renamed: {renamed}, Skipped: {skipped}."));
    Ok(())
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Source {
    Explicit,
    Glob,
}

struct Candidate {
    path: PathBuf,
    kind: Source,
}

impl Candidate {
    fn explicit(path: PathBuf) -> Self {
        Self { path, kind: Source::Explicit }
    }
    fn glob(path: PathBuf) -> Self {
        Self { path, kind: Source::Glob }
    }
}

/// Returns true if a string contains glob characters.
fn is_glob_like(s: &str) -> bool {
    s.chars().any(|c| matches!(c, '*' | '?' | '['))
}

/// Compute where to start searching for a glob pattern.
/// We take the substring *before* the first glob metachar, then truncate to the last path separator.
/// If no separator exists before the metachar, we use the current directory.
fn infer_search_root(pattern: &str) -> Result<PathBuf> {
    let bytes = pattern.as_bytes();
    let mut first_meta_idx: Option<usize> = None;
    for (i, b) in bytes.iter().enumerate() {
        if matches!(*b as char, '*' | '?' | '[') {
            first_meta_idx = Some(i);
            break;
        }
    }

    if let Some(mi) = first_meta_idx {
        // find last separator before meta
        let before = &pattern[..mi];
        let sep_idx = before.rmatch_indices(|c: char| c == '/' || c == '\\').next().map(|x| x.0);
        let root = if let Some(si) = sep_idx {
            &before[..=si] // include the separator
        } else {
            ""
        };
        let pb = if root.is_empty() {
            std::env::current_dir()?
        } else {
            PathBuf::from(root)
        };
        Ok(pb)
    } else {
        // No meta -> treat as literal path
        Ok(PathBuf::from(pattern))
    }
}

/// Decide whether `path` matches `pattern`'s compiled matcher.
/// - For absolute patterns, match against absolute paths.
/// - For relative patterns, match against paths relative to current_dir when possible.
fn is_match(m: &GlobMatcher, pattern: &str, path: &Path) -> bool {
    if Path::new(pattern).is_absolute() {
        // try absolute
        match path.canonicalize() {
            Ok(abs) => m.is_match(abs),
            Err(_) => m.is_match(path),
        }
    } else {
        // try relative to CWD
        if let Ok(cwd) = std::env::current_dir() {
            if let Ok(rel) = path.strip_prefix(&cwd) {
                return m.is_match(rel);
            }
        }
        m.is_match(path)
    }
}

/// Build the destination path for a rename given a replacement expression.
/// '{}' is replaced by the original name *without extension* (the "stem").
/// - If the replacement produces an absolute path, use it as-is.
/// - Otherwise, place it next to the original item.
fn compute_target_path(original: &Path, replacement_expr: &str) -> Result<PathBuf> {
    // Use stem for files and directories alike (dirs may have dots; stem handles that).
    let stem = original
        .file_stem()
        .or_else(|| original.file_name())
        .and_then(|s| s.to_str())
        .unwrap_or("");

    let built = replacement_expr.replace("{}", stem);

    let target = Path::new(&built);
    if target.is_absolute() {
        Ok(target.to_path_buf())
    } else {
        Ok(original
            .parent()
            .map(|p| p.join(target))
            .unwrap_or_else(|| PathBuf::from(built)))
    }
}

/// Canonicalize path, or return original if canonicalization fails (e.g., permission issues).
fn canonical_or_self(p: &Path) -> PathBuf {
    p.canonicalize().unwrap_or_else(|_| p.to_path_buf())
}
