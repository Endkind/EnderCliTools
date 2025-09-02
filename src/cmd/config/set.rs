use anyhow::Result;

use crate::args::config::set::{SetArgs, SetCommands, SetDpsArgs, SetTableArgs};
use crate::config::Config;

pub fn set(args: SetArgs) -> Result<()> {
    match args.command {
        SetCommands::Table(args) => table(args),
        SetCommands::Dps(args) => dps(args),
    }
}

fn table(args: SetTableArgs) -> Result<()> {
    let mut cfg = Config::load()?;

    if let Some(preset) = args.preset {
        cfg.table.preset = preset;
    }
    if let Some(modifier) = args.modifier {
        cfg.table.modifier = modifier;
    }

    cfg.save()
}

fn dps(args: SetDpsArgs) -> Result<()> {
    let mut cfg = Config::load()?;

    if let Some(headers) = args.headers {
        cfg.dps.headers = headers;
    }

    cfg.save()
}
