use crate::cli::CheckReposArgs;
use anyhow::{Context, Result};
use colored::Colorize;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Represents the status of a Git repository.
enum RepoStatus {
    Uncommitted,
    NoUpstream,
    NotPushed,
    Ok,
}

pub fn run(args: &CheckReposArgs) -> Result<()> {
    let mut uncommitted = Vec::new();
    let mut no_upstream = Vec::new();
    let mut not_pushed = Vec::new();

    // 1. Navigate through each of the directories passed as arguments
    for dir in &args.directories {
        // Confirm that the path is a valid direct
        if !dir.is_dir() {
            println!(
                "{} a directory: {}",
                "Warning:".yellow(),
                dir.display()
            );
            continue;
        }

        // 2. Iterate over each directory inside the path
        for entry in fs::read_dir(dir).with_context(|| format!("Failed to read directory {}", dir.display()))? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                println!("{} {}", "🔍 Checking".cyan(), path.display());
                // 3. Get directory status
                match check_repo_status(&path)? {
                    Some(RepoStatus::Uncommitted) => uncommitted.push(path),
                    Some(RepoStatus::NoUpstream) => no_upstream.push(path),
                    Some(RepoStatus::NotPushed) => not_pushed.push(path),
                    Some(RepoStatus::Ok) => (),
                    None => {
                        println!(
                            "{} {} is not a git repository",
                            "📁 Info:".yellow(),
                            path.display()
                        );
                    }
                }
            }
        }
    }

    // 4. Report to the user the results
    print_report(&uncommitted, &no_upstream, &not_pushed)?;

    Ok(())
}

/// Checks a single directory to see if it's a Git repo and what its status is.
fn check_repo_status(path: &Path) -> Result<Option<RepoStatus>> {
    // 1. Check if it's a git repository
    let is_repo = Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("rev-parse")
        .arg("--is-inside-work-tree")
        .output()?
        .status
        .success();

    if !is_repo {
        return Ok(None);
    }

    // 2. Check for uncommitted changes
    let status_output = Command::new("git").arg("-C").arg(path).arg("status").arg("--porcelain").output()?;
    if !status_output.stdout.is_empty() {
        return Ok(Some(RepoStatus::Uncommitted));
    }

    // 3. Check if upstream is set
    let upstream_output = Command::new("git").arg("-C").arg(path).arg("rev-parse").arg("--abbrev-ref").arg("--symbolic-full-name").arg("@{u}").output()?;
    if !upstream_output.status.success() {
        return Ok(Some(RepoStatus::NoUpstream));
    }

    // 4. Check for unpushed commits
    let cherry_output = Command::new("git").arg("-C").arg(path).arg("cherry").arg("-v").output()?;
    if !cherry_output.stdout.is_empty() {
        return Ok(Some(RepoStatus::NotPushed));
    }

    Ok(Some(RepoStatus::Ok))
}

/// Prints the final summary report to the console.
fn print_report(
    uncommitted: &[PathBuf],
    no_upstream: &[PathBuf],
    not_pushed: &[PathBuf],
) -> Result<()> {
    if !uncommitted.is_empty() {
        println!("{}", "\n🟡 The following directories contain uncommitted changes:".yellow());
        for dir in uncommitted {
            println!("{}", dir.display().to_string().yellow());
        }
    }

    if !no_upstream.is_empty() {
        println!("{}", "\n🚫 The following directories do not have an upstream branch set:".red());
        for dir in no_upstream {
            println!("{}", dir.display().to_string().red());
            let branch_output = Command::new("git").arg("-C").arg(dir).arg("rev-parse").arg("--abbrev-ref").arg("HEAD").output()?;
            let branch = String::from_utf8(branch_output.stdout)?.trim().to_string();
            println!("{} Remote branch 'origin/{}' exists. To link it, run:", "ℹ️".cyan(), branch);
            println!("\t{}", format!("git -C \"{}\" branch --set-upstream-to=origin/{} {}", dir.display(), branch, branch).cyan());
        }
    }

    if !not_pushed.is_empty() {
        println!("{}", "\n📤 The following directories contain changes that were committed but not yet pushed:".truecolor(135, 255, 135));
        for dir in not_pushed {
            println!("{}", dir.display().to_string().truecolor(135, 255, 135));
        }
    }

    Ok(())
}
