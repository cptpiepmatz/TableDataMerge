use crate::table::cell::Cell;
use crate::table::{FormatOptions, ParseTableError, Table};
use std::cmp;
use std::str::FromStr;

impl Table {
    pub fn from_json(raw: &str) -> Result<Table, ParseTableError> {
        let parsed_table = serde_json::from_str::<Vec<Vec<String>>>(raw);
        // TODO: handle this well
        let parsed_table = parsed_table.unwrap();

        // get width and height
        let (mut width, mut height) = (0, 0);
        for (i, row) in parsed_table.iter().enumerate() {
            for (j, _) in row.iter().enumerate() {
                width = cmp::max(j + 1, width);
            }
            height = i + 1;
        }

        let mut table = Table::new(width, height);
        for (i, row) in parsed_table.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                table.values[i][j] = Cell::from_str(cell).unwrap(); // infallible
            }
        }

        Ok(table)
    }

    pub fn to_json(&self, format_options: &FormatOptions) -> String {
        let mut str_table: Vec<Vec<String>> = Vec::with_capacity(self.height);

        for row in self.values.iter() {
            let mut str_row = Vec::with_capacity(self.width);
            for (i, cell) in row.iter().enumerate() {
                str_row.push(cell.fmt(format_options, i));
            }
            str_table.push(str_row);
        }

        serde_json::to_string_pretty(&str_table).unwrap() // only strings, should be ok
    }
}
