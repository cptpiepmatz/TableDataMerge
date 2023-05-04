use crate::table::transform::csv::ParseCsvTableError;
use crate::table::transform::json::ParseJsonTableError;
use crate::table::transform::m::ParseMTableError;

pub mod csv;
pub mod dat;
pub mod json;
pub mod m;
pub mod md;
pub mod tex;

pub enum ParseTableError {
    Csv(ParseCsvTableError),
    Json(ParseJsonTableError),
    M(ParseMTableError),
}
