use crate::config::model::table::{TableModifiers, TablePresets};
use comfy_table::{Cell, ContentArrangement, Table};

pub type TableRow = Vec<String>;

pub fn build_table(
    headers: &TableRow,
    rows: Option<&[TableRow]>,
    preset: Option<&TablePresets>,
    modifier: Option<&TableModifiers>,
) -> Table {
    let mut table = Table::new();
    let preset = preset.unwrap_or(&TablePresets::Utf8Full).to_preset();
    let modifier = modifier
        .unwrap_or(&TableModifiers::Utf8RoundCorners)
        .to_modifier();

    table
        .load_preset(preset)
        .apply_modifier(modifier)
        .set_content_arrangement(ContentArrangement::Dynamic);

    let header_cells = headers.iter().map(Cell::new);
    table.set_header(header_cells);

    if let Some(rows) = rows {
        for row in rows {
            let cells = row.iter().map(Cell::new);
            table.add_row(cells);
        }
    }

    table
}
