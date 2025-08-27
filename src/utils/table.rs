use comfy_table:: {
    modifiers::UTF8_ROUND_CORNERS,
    presets::UTF8_FULL,
    Cell, ContentArrangement, Table,
};

pub type TableRow = Vec<String>;

pub fn build_table(headers: &TableRow, rows: Option<&[TableRow]>) -> Table {
    let mut table = Table::new();

    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
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
