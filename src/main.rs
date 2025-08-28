mod args;
mod cmd;
mod utils;
mod config;

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
