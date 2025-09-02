use crate::args::dps::DpsArgs;
use anyhow::Result;
use crate::config::Config;
use crate::utils;
use crate::utils::table::TableRow;

pub fn run(args: DpsArgs) -> Result<()> {
    let stdout = utils::docker::ps(args.all, args.headers.clone())?;
    let cfg = Config::load()?;
    let table_preset = args.table_preset.unwrap_or(cfg.table.preset);
    let table_modifier = args.table_modifier.unwrap_or(cfg.table.modifier);

    let table_headers: TableRow = if let Some(headers) = args.headers {
        headers
            .into_iter()
            .map(|h| h.display_name().into())
            .collect()
    } else {
        cfg.dps.headers
            .iter()
            .map(|h| h.display_name().into())
            .collect()
    };

    let mut table = utils::table::build_table(&table_headers, None, Some(&table_preset), Some(&table_modifier));

    for line in stdout.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let mut cols = line.split(';').map(|s| s.trim().to_string()).collect::<TableRow>();
        while cols.len() < table_headers.len() {
            cols.push("".to_string());
        }

        table.add_row(cols);
    }

    println!("{}", table);

    Ok(())
}
