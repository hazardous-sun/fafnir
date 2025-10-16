use std::path::PathBuf;
use std::process::Command;
use crate::cli::RemoteOperationArgs;
use crate::commands::git_utilities::utils::run_with_action;
use crate::utils::logger;

pub fn run(args: &RemoteOperationArgs) -> Result<(), anyhow::Error> {
    run_with_action(args, on_ok_push)
}

fn on_ok_push(path: PathBuf) -> anyhow::Result<(), anyhow::Error> {
    logger::debug(&format!("Pushing to '{}'... ", path.display()));

    let output = Command::new("git").arg("push").current_dir(path.clone()).output()?;

    if !output.status.success() {
        logger::error(&format!("Failed to push to origin in '{}'", path.display()));
    }

    Ok(())
}
