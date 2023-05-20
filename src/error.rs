use crate::table::ParseTableError;
use std::{io, process};

pub enum TdmError {
    ReadFile { path: String, error: io::Error },
    WriteFile { path: String, error: io::Error },
    DetermineFileStem { file: String },
    DetermineFileType { file: String },
    UnknownFileType { file_type: String },
    ParseTable(ParseTableError),
}

impl TdmError {
    pub fn handle(&self) -> ! {
        process::exit(1)
    }
}
