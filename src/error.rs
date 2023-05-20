use crate::table::ParseTableError;
use paris::Logger;
use std::{io, process};

#[derive(Debug)]
pub enum TdmError {
    ReadFile { path: String, error: io::Error },
    WriteFile { path: String, error: io::Error },
    DetermineFileStem { file: String },
    DetermineFileType { file: String },
    UnknownFileType { file_type: String },
    ParseTable(ParseTableError),
}

impl TdmError {
    pub fn handle(&self, logger: &mut Logger) -> ! {
        logger.done();
        logger.error(self.msg());
        process::exit(self.exit_code())
    }

    fn msg(&self) -> String {
        match self {
            TdmError::ReadFile { path, error } => {
                let error = format!("{error}");
                format!(
                    "Could not read file '{path}', {}",
                    uncapitalize_sentence(&error)
                )
            }
            TdmError::WriteFile { path, error } => {
                let error = format!("{error}");
                format!(
                    "Could not write file '{path}', {}",
                    uncapitalize_sentence(&error)
                )
            }
            TdmError::DetermineFileStem { file } => {
                format!("Could not determine file steam for '{file}'")
            }
            TdmError::DetermineFileType { file } => {
                format!("Could not determine file type for '{file}'")
            }
            TdmError::UnknownFileType { file_type } => {
                format!("Unknown file type '{file_type}' for parsing")
            }
            TdmError::ParseTable(e) => format!("Could not parse table, {e}"),
        }
    }

    fn exit_code(&self) -> i32 {
        match self {
            TdmError::ReadFile { .. } => 3,
            TdmError::WriteFile { .. } => 4,
            TdmError::DetermineFileStem { .. } => 5,
            TdmError::DetermineFileType { .. } => 6,
            TdmError::UnknownFileType { .. } => 7,
            TdmError::ParseTable(_) => 8,
        }
    }
}

fn uncapitalize_sentence(s: &str) -> String {
    let (l, r) = s.split_at(1);
    l.to_lowercase() + r
}
