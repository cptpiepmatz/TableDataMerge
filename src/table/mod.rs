use crate::cli::{Args, DecimalSeparator};
use crate::table::cell::Cell;
use crate::util::AnyRange;
use std::cmp;
use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};
use tabled::builder::Builder;

pub mod cell;
mod transform;
pub use transform::ParseTableError;

/// A general table structure that holds cells.
///
/// The `Table` struct provides the functionality to store, manipulate, and merge tables.
/// It can be read from and written to by implementing transformer functions `from_x` and `to_x`.
/// A custom implementation of the `Debug` trait provides a human-readable representation of the
/// table.
/// The struct ensures that the height and width of the table are always consistent when
/// concatenating or stacking tables.
pub struct Table {
    // TODO: in all implementations these are not used correctly but are 1 short
    height: usize,
    width: usize,
    values: VecDeque<Vec<Cell>>,
}

impl Table {
    /// Pads the bottom of the table with blank cells until it reaches the specified height.
    fn pad_bottom(&mut self, height: usize) {
        if self.height < height {
            let needed_height = height - self.height;
            for _ in 0..needed_height {
                self.values.push_back(Self::create_blank_row(self.width));
            }
        }
        self.height = height;
    }

    /// Pads the right side of the table with blank cells until it reaches the specified width.
    fn pad_right(&mut self, width: usize) {
        if self.width < width {
            let needed_width = width - self.width;
            for row in self.values.iter_mut() {
                for _ in 0..needed_width {
                    row.push(Cell::Blank);
                }
            }
        }
        self.width = width;
    }

    /// Concatenates another table to the right side of the current table.
    ///
    /// This operation modifies both tables to ensure they have the same height.
    /// The other table is consumed and becomes invalid after this operation.
    pub fn concat(&mut self, mut other: Self) {
        // other is pulled into and therefore useless afterwards
        let height = cmp::max(self.height, other.height);
        self.pad_bottom(height);
        other.pad_bottom(height);
        self.values
            .iter_mut()
            .zip(other.values.iter_mut())
            .for_each(|(self_values, other_values)| self_values.append(other_values));
        self.width += other.width;
    }

    /// Stacks another table on top of the current table.
    ///
    /// This operation modifies both tables to ensure they have the same width.
    /// The other table is consumed and becomes invalid after this operation.
    pub fn stack(&mut self, mut other: Self) {
        let width = cmp::max(self.width, other.width);
        self.pad_right(width);
        other.pad_right(width);
        while let Some(row) = other.values.pop_front() {
            self.values.push_back(row);
        }
        self.height += other.height;
    }

    /// Creates a new row of blank cells with the specified width.
    fn create_blank_row(width: usize) -> Vec<Cell> {
        let mut row = Vec::with_capacity(width);
        row.resize(width, Cell::Blank);
        row
    }
}

/// Custom implementation of the `Debug` trait for `Table`.
///
/// This implementation provides a human-readable representation of the table.
impl Debug for Table {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut table_builder = Builder::with_capacity(self.height * self.width);
        for row in self.values.iter() {
            table_builder.push_record(row.iter().map(|v| format!("{v:?}")));
        }
        let table = table_builder.build();
        write!(f, "{table}")
    }
}

impl From<VecDeque<Vec<Cell>>> for Table {
    fn from(mut values: VecDeque<Vec<Cell>>) -> Self {
        let (mut width, mut height) = (0, 0);
        for (i, row) in values.iter().enumerate() {
            height = i;
            for (j, _) in row.iter().enumerate() {
                width = cmp::max(width, j + 1)
            }
        }
        for row in values.iter_mut() {
            row.resize(width, Cell::Blank);
        }
        Table {
            height,
            width,
            values,
        }
    }
}

#[derive(Debug, Default)]
pub struct FormatOptions {
    precision: Option<u16>,
    exponent: bool,
    decimal_sep: DecimalSeparator,
    sign: bool,
    hline: bool,
    csv_sep: String,
    prefix: Vec<(AnyRange<usize>, String)>,
    suffix: Vec<(AnyRange<usize>, String)>,
}

impl From<Args> for FormatOptions {
    fn from(value: Args) -> Self {
        FormatOptions {
            precision: value.precision,
            exponent: value.exponent,
            decimal_sep: value.decimal_sep,
            sign: value.sign,
            hline: value.hline,
            csv_sep: value.csv_sep,
            prefix: value.prefix,
            suffix: value.suffix,
        }
    }
}
