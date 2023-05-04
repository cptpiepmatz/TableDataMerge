use crate::table::cell::Cell;
use crate::table::transform::ParseTableError;
use crate::table::{FormatOptions, Table};
use serde_json::Error;
use std::collections::VecDeque;
use std::str::FromStr;

impl Table {
    pub fn from_json(raw: &str, _: &Option<String>) -> Result<Table, ParseTableError> {
        let parsed_table: VecDeque<Vec<String>> = serde_json::from_str(raw)?;
        let table: VecDeque<Vec<Cell>> = parsed_table
            .iter()
            .map(|r| {
                r.iter()
                    .map(|c| Cell::from_str(c).expect("infallible"))
                    .collect()
            })
            .collect();
        Ok(table.into())
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

pub type ParseJsonTableError = serde_json::Error;

impl From<serde_json::Error> for ParseTableError {
    fn from(value: Error) -> Self {
        ParseTableError::Json(value)
    }
}
