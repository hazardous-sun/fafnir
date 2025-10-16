use std::path::PathBuf;
use std::process::Command;
use crate::cli::RemoteOperationArgs;
use crate::commands::git_utilities::utils::run_with_action;
use crate::utils::logger;

pub fn run(args: &RemoteOperationArgs) -> Result<(), anyhow::Error> {
    run_with_action(args, on_ok_pull)
}

fn on_ok_pull(path: PathBuf) -> anyhow::Result<(), anyhow::Error> {
    logger::debug(&format!("Pulling in '{}'... ", path.display()));

    let output = Command::new("git").arg("pull").current_dir(path.clone()).output()?;

    if !output.status.success() {
        logger::error(&format!("Failed to pull from the origin in '{}'", path.display()));
    }

    Ok(())
}
