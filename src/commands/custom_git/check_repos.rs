use crate::cli::CheckReposArgs;
use crate::commands::custom_git::utils::run_with_action;
use anyhow::Result;
use std::path::PathBuf;

pub fn run(args: &CheckReposArgs) -> Result<()> {
    run_with_action(args, on_ok_check)
}

fn on_ok_check(path: PathBuf) -> Result<(), anyhow::Error> {
    Ok(())
}
