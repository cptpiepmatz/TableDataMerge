use std::cmp;

use csv::ReaderBuilder;

use crate::table::{FormatOptions, ParseTableError};

use crate::table::Table;

impl Table {
    pub fn from_csv(raw: &str, additional_data: &Option<String>) -> Result<Table, ParseTableError> {
        // determine delimiter
        let delimiter = match additional_data.clone().and_then(|s| s.chars().next()) {
            None => ';',
            Some(c) => c,
        };

        // get width and height
        let mut reader = ReaderBuilder::new()
            .has_headers(false)
            .delimiter(delimiter as u8)
            .from_reader(raw.as_bytes());
        let (mut width, mut height) = (0, 0);
        for record in reader.records() {
            height += 1;
            let record = match record {
                Err(_) => panic!(),
                Ok(record) => record,
            };
            for (i, _) in record.iter().enumerate() {
                width = cmp::max(width, i + 1);
            }
        }

        // construct table
        let mut table = Table::new(width, height);
        let mut reader = ReaderBuilder::new()
            .has_headers(false)
            .delimiter(delimiter as u8)
            .from_reader(raw.as_bytes());
        for (i, record) in reader.records().enumerate() {
            for (j, item) in record.unwrap().iter().enumerate() {
                table.values[i][j] = item.parse().unwrap();
            }
        }
        Ok(table)
    }

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
