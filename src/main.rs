use anyhow::{Ok, Result};
use clap::Parser;

mod cli;
mod wc;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();

    wc::wc(cli)?;

    Ok(())
}
