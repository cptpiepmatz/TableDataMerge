use std::cmp;
use std::collections::VecDeque;
use std::str::FromStr;

use csv::ReaderBuilder;
use lazy_static::lazy_static;
use regex::Regex;

use crate::table::{FormatOptions, ParseTableError};
use crate::table::cell::Cell;

use crate::table::Table;

impl Table {
    pub fn from_m(raw: &str, _: &Option<String>) -> Result<Table, ParseTableError> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"\[([^\[\]]+)\]")
                    .unwrap();
        }

        let relevant_part = RE.find(raw).unwrap().as_str();
        let values = &relevant_part[1..(relevant_part.len() - 1)];

        let mut raw_table = VecDeque::new();
        let rows = values.split(";").map(str::trim);
        for row in rows {
            let mut row_items = Vec::new();
            for cell in row.split(" ") {
                row_items.push(Cell::from_str(cell).unwrap());
            }
            raw_table.push_back(row_items);
        }

        Ok(Table::from(raw_table))
    }
}
