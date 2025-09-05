use crate::args::config::Normalizable;
use crate::args::config::reset::{ResetArgs, ResetCommands, ResetDpsArgs, ResetTableArgs};
use crate::config::Config;
use crate::config::model::dps::DpsConfig;
use crate::config::model::table::TableConfig;
use anyhow::Result;

pub fn reset(args: ResetArgs) -> Result<()> {
    match Config::backup()? {
        Some(backup_path) => println!("Backup created at: {}", backup_path),
        None => println!("No existing config to back up."),
    };

    if let Some(command) = args.command {
        match command {
            ResetCommands::Table(args) => table(args),
            ResetCommands::Dps(args) => dps(args),
        }
    } else {
        Config::default().save()
    }
}

fn table(args: ResetTableArgs) -> Result<()> {
    let mut cfg = Config::load()?;
    let args = args.normalize();

    if args.all {
        cfg.table = TableConfig::default();
        return cfg.save();
    }

    if args.preset {
        cfg.table.preset = TableConfig::default().preset;
    }
    if args.modifier {
        cfg.table.modifier = TableConfig::default().modifier;
    }

    cfg.save()
}

fn dps(args: ResetDpsArgs) -> Result<()> {
    let mut cfg = Config::load()?;
    let args = args.normalize();

    if args.all {
        cfg.dps = DpsConfig::default();
        return cfg.save();
    }

    if args.headers {
        cfg.dps.headers = DpsConfig::default().headers;
    }

    cfg.save()
}
