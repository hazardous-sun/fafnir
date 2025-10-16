use std::path::PathBuf;
use std::process::Command;
use crate::cli::CheckReposArgs;
use crate::commands::custom_git::utils::run_with_action;
use crate::utils::logger;

pub fn run(args: &CheckReposArgs) -> Result<(), anyhow::Error> {
    run_with_action(args, on_ok_push)
}

fn on_ok_push(path: PathBuf) -> anyhow::Result<(), anyhow::Error> {
    logger::debug(&format!("Pushing to '{}'... ", path.display()));

    let output = Command::new("git").arg("push").current_dir(path).output()?;

    if !output.status.success() {
        logger::error("Failed to push to the origin... ");
    }

    Ok(())
}
