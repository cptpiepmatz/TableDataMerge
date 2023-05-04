use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::table::transform::csv::ParseCsvTableError;
use crate::table::transform::json::ParseJsonTableError;
use crate::table::transform::m::ParseMTableError;

pub mod csv;
pub mod dat;
pub mod json;
pub mod m;
pub mod md;
pub mod tex;

/// Error if parsing tables fail.
#[derive(Debug)]
pub enum ParseTableError {
    Csv(ParseCsvTableError),
    Json(ParseJsonTableError),
    M(ParseMTableError),
}

impl Display for ParseTableError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseTableError::Csv(c) => c.fmt(f),
            ParseTableError::Json(j) => j.fmt(f),
            ParseTableError::M(m) => m.fmt(f)
        }
    }
}

impl Error for ParseTableError {}
