use crate::args::config::{ConfigArgs, ConfigCommands};
use anyhow::Result;
use set::set;
use get::get;
use reset::reset;

mod set;
mod get;
mod reset;

pub fn run(args: ConfigArgs) -> Result<()> {
    match args.command {
        ConfigCommands::Set(args) => {
            set(args)
        }
        ConfigCommands::Get(args) => {
            get(args)
        }
        ConfigCommands::Reset(args) => {
            reset(args)
        }
    }
}
