mod collector;

use clap::Parser;
use anyhow::Result;

fn main() -> Result<()> {
    collector::run()?;

    Ok(())
}
