use std::cmp;

use crate::table::cell::Cell;
use crate::table::Table;
use crate::table::{FormatOptions, ParseTableError};

impl Table {
    pub fn from_dat(raw: &str) -> Result<Table, ParseTableError> {
        // compute width and height of table
        let mut height: usize = 0;
        let mut width: usize = 0;
        for (i, line) in raw.lines().enumerate() {
            for (j, _) in line.trim().split('\t').enumerate() {
                width = cmp::max(j + 1, width);
            }
            height = i + 1;
        }

        // construct actual table and fill data
        let (height, width) = (height, width);
        let mut table = Table::new(width, height);
        for (i, line) in raw.lines().enumerate() {
            for (j, cell) in line.trim().split('\t').enumerate() {
                let cell: Cell = cell.parse().unwrap(); // infallible
                table.values[i][j] = cell;
            }
        }

        Ok(table)
    }

    pub fn to_dat(&self, format_options: &FormatOptions) -> String {
        let mut output = String::new();
        for (i, row) in self.values.iter().enumerate() {
            output +=
                &*(itertools::join(row.iter().map(|c| c.fmt(format_options, i)), "\t") + "\n");
        }
        output
    }
}
