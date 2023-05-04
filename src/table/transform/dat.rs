use crate::table::cell::Cell;
use crate::table::Table;
use crate::table::{FormatOptions, ParseTableError};
use std::collections::VecDeque;
use std::str::FromStr;

impl Table {
    /// Construct a table from the contents of a dat file.
    pub fn from_dat(raw: &str, _: &Option<String>) -> Result<Table, ParseTableError> {
        let table: VecDeque<Vec<Cell>> = raw
            .lines()
            .map(|l| {
                l.trim()
                    .split('\t')
                    .map(Cell::from_str)
                    .map(Result::unwrap)
                    .collect()
            })
            .collect();
        Ok(table.into())
    }

    /// Construct a dat representation.
    pub fn to_dat(&self, format_options: &FormatOptions) -> String {
        let mut output = String::new();
        for (i, row) in self.values.iter().enumerate() {
            output +=
                &*(itertools::join(row.iter().map(|c| c.fmt(format_options, i)), "\t") + "\n");
        }
        output
    }
}
