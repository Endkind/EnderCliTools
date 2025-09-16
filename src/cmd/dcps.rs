use crate::args::dcps::DcpsArgs;
use crate::config::Config;
use crate::config::model::dcps::DcpsHeader;
use crate::utils;
use crate::utils::table::TableRow;
use anyhow::Result;

pub fn run(args: DcpsArgs) -> Result<()> {
    let stdout = utils::docker::compose::ps(
        args.all,
        args.headers.as_deref(),
        args.add_headers.as_deref(),
        args.no_trunc,
        args.no_orphans,
        args.quiet,
        args.services,
        args.status.as_deref(),
    )?;
    let cfg = Config::load()?;
    let table_preset = args.table_preset.unwrap_or(cfg.table.preset);
    let table_modifier = args.table_modifier.unwrap_or(cfg.table.modifier);

    let mut table_headers: TableRow = if args.quiet {
        vec![DcpsHeader::Id.display_name().into()]
    } else if args.services {
        vec![DcpsHeader::Service.display_name().into()]
    } else {
        if let Some(headers) = args.headers {
            headers
                .into_iter()
                .map(|h| h.display_name().into())
                .collect()
        } else {
            cfg.dcps
                .headers
                .iter()
                .map(|h| h.display_name().into())
                .collect()
        }
    };

    if let Some(add_headers) = args.add_headers {
        for add_header in add_headers {
            table_headers.push(add_header.display_name().into());
        }
    };

    let mut table = utils::table::build_table(
        &table_headers,
        None,
        Some(&table_preset),
        Some(&table_modifier),
    );

    for line in stdout.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let mut cols = line
            .split(';')
            .map(|s| s.trim().into())
            .collect::<TableRow>();
        while cols.len() < table_headers.len() {
            cols.push("".into());
        }

        table.add_row(cols);
    }

    println!("{}", table);

    Ok(())
}
