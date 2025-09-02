use crate::args::dps::DpsArgs;
use anyhow::Result;
use crate::config::Config;
use crate::config::model::dps::DpsHeader;
use crate::utils;
use crate::utils::table::TableRow;

pub fn run(args: DpsArgs) -> Result<()> {
    let stdout = utils::docker::ps(
        args.all,
        args.headers.clone(),
        args.add_headers.clone(),
        args.last,
        args.latest,
        args.no_trunc,
        args.quiet,
        args.size
    )?;
    let cfg = Config::load()?;
    let table_preset = args.table_preset.unwrap_or(cfg.table.preset);
    let table_modifier = args.table_modifier.unwrap_or(cfg.table.modifier);

    let mut table_headers: TableRow = if args.quiet {
        vec![DpsHeader::Id.display_name().into()]
    } else {
        if let Some(headers) = args.headers {
            headers
                .into_iter()
                .map(|h| h.display_name().into())
                .collect()
        } else {
            cfg.dps.headers
                .iter()
                .map(|h| h.display_name().into())
                .collect()
        }
    };

    if args.size {
        table_headers.push(DpsHeader::Size.display_name().into());
    }

    if let Some(add_headers) = args.add_headers {
        for add_header in add_headers {
            table_headers.push(add_header.display_name().into());
        }
    }

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
