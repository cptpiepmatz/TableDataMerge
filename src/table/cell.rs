use std::fmt::{Debug, Formatter};
use std::str::FromStr;

use crate::table::FormatOptions;

#[derive(Default, Clone)]
pub enum Cell {
    Int(i128),
    Float(f64),
    Str(String),

    #[default]
    Blank,
}

impl Cell {
    pub fn format(&self, format_options: &FormatOptions) -> String {
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
