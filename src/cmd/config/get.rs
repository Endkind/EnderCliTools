use anyhow::Result;
use comfy_table::Table;
use crate::args::config::get::{GetArgs, GetCommands, GetDpsArgs, GetTableArgs};
use crate::args::config::Normalizable;
use crate::config::Config;
use crate::utils::table::{build_table, TableRow};

pub fn get(args: GetArgs) -> Result<()> {
    let cfg = Config::load()?;
    let table_headers: TableRow = vec!["OPTION".into(), "VALUE".into()];
    let mut config_table = build_table(&table_headers, None, Some(&cfg.table.preset), Some(&cfg.table.modifier));

    if let Some(command) = args.command {
        match command {
            GetCommands::Table(args) => {
                config_table = table(config_table, args)?;
            }
            GetCommands::Dps(args) => {
                config_table = dps(config_table, args)?;
            }
        }
    } else {
        config_table = table(config_table, GetTableArgs::default())?;
        config_table = dps(config_table, GetDpsArgs::default())?;
    }

    println!("{}", config_table);

    Ok(())
}

fn table(mut config_table: Table, mut args: GetTableArgs) -> Result<Table> {
    let cfg = Config::load()?;
    args = args.normalize();

    if args.preset || args.all {
        let table_row: TableRow = vec!["table.preset".into(), cfg.table.preset.to_string()];
        config_table.add_row(table_row);
    }
    if args.modifier || args.all {
        let table_row: TableRow = vec!["table.modifier".into(), cfg.table.modifier.to_string()];
        config_table.add_row(table_row);
    }

    Ok(config_table)
}

fn dps(mut config_table: Table, mut args: GetDpsArgs) -> Result<Table> {
    let cfg = Config::load()?;
    args = args.normalize();

    if args.headers || args.all {
        let table_row: TableRow = vec!["dps.headers".into(), format!("{:?}", cfg.dps.headers)];
        config_table.add_row(table_row);
    }

    Ok(config_table)
}
