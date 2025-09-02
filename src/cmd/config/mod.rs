use crate::args::config::{ConfigArgs, ConfigCommands};
use anyhow::Result;
use get::get;
use reset::reset;
use set::set;

mod get;
mod reset;
mod set;

pub fn run(args: ConfigArgs) -> Result<()> {
    match args.command {
        ConfigCommands::Set(args) => set(args),
        ConfigCommands::Get(args) => get(args),
        ConfigCommands::Reset(args) => reset(args),
    }
}
