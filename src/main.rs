#![allow(clippy::collapsible_else_if)]

mod args;
mod cmd;
mod config;
mod utils;

use anyhow::Result;
use args::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        args::Commands::Dps(opts) => {
            cmd::dps::run(opts)?;
        }
        args::Commands::Config(opts) => {
            cmd::config::run(opts)?;
        }
    }

    Ok(())
}
