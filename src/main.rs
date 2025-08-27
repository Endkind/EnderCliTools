mod args;
mod cmd;
mod utils;

use anyhow::Result;
use args::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        args::Commands::Dps(opts) => {
            cmd::dps::run(opts)?;
        }
    }

    Ok(())
}
