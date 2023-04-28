use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use tabled::builder::Builder;

pub struct Table {
    height: usize,
    width: usize,
    values: Vec<Vec<Cell>>,
}

impl Table {
    pub fn new(width: usize, height: usize) -> Table {
        let mut rows = Vec::with_capacity(height);
        for _ in 0..height {
            let mut row = Vec::with_capacity(width);
            row.resize(width, Cell::Blank);
            rows.push(row);
        }
        Table {
            height,
            width,
            values: rows,
        }
    }
}

impl Debug for Table {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut table_builder = Builder::with_capacity(self.height * self.width);
        for row in self.values.iter() {
            table_builder.push_record(row.iter().map(|v| v.to_string()));
        }
        let table = table_builder.build();
        write!(f, "{table}")
    }
}

#[derive(Debug, Default, Clone)]
pub enum Cell {
    Int(i128),
    Float(f64),
    Str(String),

    #[default]
    Blank,
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

impl Display for Cell {
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
