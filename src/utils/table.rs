use comfy_table:: {Cell, ContentArrangement, Table};
use crate::config::model::{TableModifiers, TablePresets};

pub type TableRow = Vec<String>;

pub fn build_table(headers: &TableRow, rows: Option<&[TableRow]>, preset: Option<&TablePresets>, modifier: Option<&TableModifiers>) -> Table {
    let mut table = Table::new();
    let preset = preset.unwrap_or(&TablePresets::UTF8_FULL).to_preset();
    let modifier = modifier.unwrap_or(&TableModifiers::UTF8_ROUND_CORNERS).to_modifier();

    table
        .load_preset(preset)
        .apply_modifier(modifier)
        .set_content_arrangement(ContentArrangement::Dynamic);

    let header_cells = headers.iter().map(|h| Cell::new(h));
    table.set_header(header_cells);

    if let Some(rows) = rows {
        for row in rows {
            let cells = row.iter().map(|c| Cell::new(c));
            table.add_row(cells);
        };
    }

    table
}
