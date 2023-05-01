use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use format_num::NumberFormat;
use crate::cli::DecimalSeparator;

use crate::table::FormatOptions;

#[derive(Default, Clone)]
pub enum Cell {
    Int(i32),
    Float(f64),
    Str(String),

    #[default]
    Blank,
}

impl Cell {
    pub fn fmt(&self, format_options: &FormatOptions) -> String {
        match self {
            Cell::Int(v) => Cell::fmt_num(*v, format_options),
            Cell::Float(v) => Cell::fmt_num(*v, format_options),
            Cell::Str(s) => s.to_owned(),
            Cell::Blank => String::from("")
        }
    }

    fn fmt_num<T>(value: T, format_options: &FormatOptions) -> String
        where T: Into<f64> + Display + PartialOrd + Copy
    {
        let nf = NumberFormat::new();
        let formatted = match (format_options.precision, format_options.exponent, format_options.sign) {
            (None, false, true) if value.into() >= 0.0 => format!("+{value}"),
            (None, false, _) => value.to_string(),
            (None, true, false) => nf.format("e", value),
            (None, true, true) => nf.format("+e", value),
            (Some(p), false, false) => nf.format(format!(".{p}f").as_str(), value),
            (Some(p), false, true) => nf.format(format!("+.{p}f").as_str(), value),
            (Some(p), true, false) => nf.format(format!(".{p}e").as_str(), value),
            (Some(p), true, true) => nf.format(format!("+.{p}e").as_str(), value)
        };

        match format_options.decimal_sep {
            DecimalSeparator::Dot => formatted,
            DecimalSeparator::Comma => formatted.replace(".", ",")
        }
    }
}

impl FromStr for Cell {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(int) = s.parse::<i32>() {
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
