use crate::table::cell::Cell;
use crate::table::transform::ParseTableError;
use crate::table::Table;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::VecDeque;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

impl Table {
    pub fn from_m(raw: &str, _: &Option<String>) -> Result<Table, ParseTableError> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\[([^\[\]]+)\]").expect("should be valid regex");
        }

        let relevant_part = RE.find(raw).ok_or(ParseMTableError)?.as_str();
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

#[derive(Debug)]
pub struct ParseMTableError;

impl Display for ParseMTableError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "could not find relevant part")
    }
}

impl Error for ParseMTableError {}

impl From<ParseMTableError> for ParseTableError {
    fn from(value: ParseMTableError) -> Self {
        ParseTableError::M(value)
    }
}
