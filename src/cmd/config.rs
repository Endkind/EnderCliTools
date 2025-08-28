use crate::args::config::{ConfigArgs, ConfigCommands, GetConfigArgs, ResetConfigArgs, SetConfigArgs};
use anyhow::Result;
use crate::config::Config;
use crate::utils;

pub fn run(args: ConfigArgs) -> Result<()> {
    match args.command {
        ConfigCommands::Set(args) => {
            set(args)
        }
        ConfigCommands::Reset(args) => {
            reset(args)
        }
        ConfigCommands::Get(args) => {
            get(args)
        }
    }
}

fn set(args: SetConfigArgs) -> Result<()> {
    let mut cfg = Config::load()?;

    if let Some(preset) = args.table_preset {
        cfg.table.preset = preset;
    }

    if let Some(modifier) = args.table_modifier {
        cfg.table.modifier = modifier;
    }

    cfg.save()
}

fn reset(args: ResetConfigArgs) -> Result<()> {
    let mut cfg = Config::load()?;

    if args.all {
        cfg = Config::default();
    }

    if args.table_preset {
        cfg.table.preset = Config::default().table.preset;
    }

    if args.table_modifier {
        cfg.table.modifier = Config::default().table.modifier;
    }

    cfg.save()
}

fn get(mut args: GetConfigArgs) -> Result<()> {
    args = args.mormalize();
    let cfg = Config::load()?;
    let table_headers: utils::table::TableRow = vec!["OPTION".into(), "VALUE".into()];
    let mut table = utils::table::build_table(&table_headers, None, Some(&cfg.table.preset), Some(&cfg.table.modifier));

    if args.table_preset || args.all {
        let table_row: utils::table::TableRow = vec!["table.preset".into(), cfg.table.preset.to_string()];
        table.add_row(table_row);
    }

    if args.table_modifier || args.all {
        let table_row: utils::table::TableRow = vec!["table.modifier".into(), cfg.table.modifier.to_string()];
        table.add_row(table_row);
    }

    println!("{}", table);

    Ok(())
}
