use crate::args::config::Normalizable;
use crate::args::config::get::{GetArgs, GetCommands, GetDpsArgs, GetTableArgs};
use crate::config::Config;
use crate::utils::table::{TableRow, build_table};
use anyhow::Result;
use comfy_table::Table;

pub fn get(args: GetArgs) -> Result<()> {
    let cfg = Config::load()?;
    let table_headers: TableRow = vec!["OPTION".into(), "VALUE".into()];
    let mut config_table = build_table(
        &table_headers,
        None,
        Some(&cfg.table.preset),
        Some(&cfg.table.modifier),
    );

    if let Some(command) = args.command {
        match command {
            GetCommands::Table(args) => {
                table(&mut config_table, &cfg, args)?;
            }
            GetCommands::Dps(args) => {
                dps(&mut config_table, &cfg, args)?;
            }
        }
    } else {
        table(&mut config_table, &cfg, GetTableArgs::default())?;
        dps(&mut config_table, &cfg, GetDpsArgs::default())?;
    }

    println!("{}", config_table);

    Ok(())
}

fn table(config_table: &mut Table, cfg: &Config, args: GetTableArgs) -> Result<()> {
    let args = args.normalize();

    if args.preset || args.all {
        let table_row: TableRow = vec!["table.preset".into(), cfg.table.preset.to_string()];
        config_table.add_row(table_row);
    }
    if args.modifier || args.all {
        let table_row: TableRow = vec!["table.modifier".into(), cfg.table.modifier.to_string()];
        config_table.add_row(table_row);
    }

    Ok(())
}

fn dps(config_table: &mut Table, cfg: &Config, args: GetDpsArgs) -> Result<()> {
    let args = args.normalize();

    if args.headers || args.all {
        let table_row: TableRow = vec!["dps.headers".into(), format!("{:?}", cfg.dps.headers)];
        config_table.add_row(table_row);
    }

    Ok(())
}
