use crate::table::cell::Cell;
use crate::table::transform::ParseTableError;
use crate::table::FormatOptions;
use crate::table::Table;
use csv::ReaderBuilder;
use std::collections::VecDeque;
use std::str::FromStr;

impl Table {
    /// Construct a table from the contents of a csv file.
    pub fn from_csv(raw: &str, additional_data: &Option<String>) -> Result<Table, ParseTableError> {
        // determine delimiter
        let delimiter = additional_data.clone().and_then(|s| s.chars().next()).unwrap_or(';');

        let mut reader = ReaderBuilder::new()
            .has_headers(false)
            .delimiter(delimiter as u8)
            .from_reader(raw.as_bytes());
        let table: Result<VecDeque<Vec<Cell>>, ParseCsvTableError> = reader
            .records()
            .map(|r| {
                r.map(|r| {
                    r.iter()
                        .map(|i| Cell::from_str(i).expect("infallible"))
                        .collect()
                })
            })
            .collect();
        Ok(table?.into())
    }

    /// Construct a csv representation.
    pub fn to_csv(&self, format_options: &FormatOptions) -> String {
        let delimiter = format_options.csv_sep.as_str();
        itertools::join(
            self.values.iter().map(|row| {
                itertools::join(
                    row.iter()
                        .enumerate()
                        .map(|(i, c)| c.fmt(format_options, i)),
                    delimiter,
                )
            }),
            "\n",
        )
    }
}

pub type ParseCsvTableError = csv::Error;

impl From<ParseCsvTableError> for ParseTableError {
    fn from(value: ParseCsvTableError) -> Self {
        ParseTableError::Csv(value)
    }
}
