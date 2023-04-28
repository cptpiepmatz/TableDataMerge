use std::cmp;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use tabled::builder::Builder;
use crate::Args;

mod dat;

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
        self.values.iter_mut().zip(other.values.iter_mut()).for_each(|(self_values, other_values)| self_values.append(other_values));
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

#[derive(Default, Clone)]
pub enum Cell {
    Int(i128),
    Float(f64),
    Str(String),

    #[default]
    Blank,
}

impl Cell {
    fn format(&self, format_options: &FormatOptions) -> String {
        match self {
            Self::Str(s) => s.to_string(),
            Self::Blank => String::new(),
            Self::Int(i) => match format_options {
                FormatOptions {
                    precision: Some(_),
                    sign: true,
                    comma: true,
                    ..
                } if *i >= 0 => format!("+{}", (*i as f64).to_string().replace(".", ",")),
                FormatOptions {
                    precision: Some(_),
                    sign: true,
                    ..
                } if *i >= 0 => format!("+{}", (*i as f64)),
                FormatOptions {
                    precision: Some(_),
                    comma: true,
                    ..
                } => (*i as f64).to_string().replace(".", ","),
                FormatOptions {
                    precision: Some(_), ..
                } => (*i as f64).to_string(),
                FormatOptions { sign: true, .. } if *i >= 0 => format!("+{i}"),
                FormatOptions { .. } => i.to_string(),
            },
            // TODO: add more cases
            Self::Float(f) => f.to_string(),
        }
    }
}

impl FromStr for Cell {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(int) = s.parse::<i128>() {
            return Ok(Cell::Int(int));
        }

        if let Ok(float) = s.parse::<f64>() {
            return Ok(Cell::Float(float));
        }

        if let Ok(float) = s.replace(',', ".").parse::<f64>() {
            return Ok(Cell::Float(float));
        }

        if s.trim().chars().count() == 0 {
            return Ok(Cell::Blank);
        }

        Ok(Cell::Str(s.to_string()))
    }
}

impl Debug for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(v) => write!(f, "{v}i"),
            Self::Float(v) => write!(f, "{v}f"),
            Self::Str(v) => write!(f, "{v}"),
            Self::Blank => write!(f, "[]"),
        }
    }
}

pub enum ParseTableError {}

#[derive(Debug, Default)]
pub struct FormatOptions {
    precision: Option<u16>,
    comma: bool,
    dot: bool,
    scientific: bool,
    sign: bool,
    math_mode: bool,
    hline: bool,
    sep: Option<String>,
}

impl From<Args> for FormatOptions {
    fn from(value: Args) -> Self {
        FormatOptions {
            precision: value.precision,
            comma: value.comma,
            dot: value.dot,
            scientific: value.scientific,
            sign: value.sign,
            math_mode: value.math_mode,
            hline: value.hline,
            sep: value.sep,
        }
    }
}
