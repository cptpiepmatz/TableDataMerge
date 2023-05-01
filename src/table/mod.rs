use std::cmp;
use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};

use tabled::builder::Builder;

use crate::cli::{Args, DecimalSeparator};
use crate::table::cell::Cell;
use crate::util::AnyRange;

pub mod cell;
mod transform;

pub enum ParseTableError {}

pub struct Table {
    height: usize,
    width: usize,
    values: VecDeque<Vec<Cell>>,
}

impl Table {
    pub fn new(width: usize, height: usize) -> Table {
        let mut rows = VecDeque::with_capacity(height);
        for _ in 0..height {
            rows.push_back(Self::create_blank_row(width));
        }
        Table {
            height,
            width,
            values: rows,
        }
    }

    fn pad_bottom(&mut self, height: usize) {
        if self.height < height {
            let needed_height = height - self.height;
            for _ in 0..needed_height {
                self.values.push_back(Self::create_blank_row(self.width));
            }
        }
        self.height = height;
    }

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

    pub fn concat(&mut self, mut other: Self) {
        // other is pulled into and therefore useless afterwards
        let height = cmp::max(self.height, other.height);
        self.pad_bottom(height);
        other.pad_bottom(height);
        self.values
            .iter_mut()
            .zip(other.values.iter_mut())
            .for_each(|(self_values, other_values)| self_values.append(other_values));
        self.width = self.width + other.width;
    }

    pub fn stack(&mut self, mut other: Self) {
        let width = cmp::max(self.width, other.width);
        self.pad_right(width);
        other.pad_right(width);
        while let Some(row) = other.values.pop_front() {
            self.values.push_back(row);
        }
        self.height = self.height + other.height;
    }

    fn create_blank_row(width: usize) -> Vec<Cell> {
        let mut row = Vec::with_capacity(width);
        row.resize(width, Cell::Blank);
        row
    }
}

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
