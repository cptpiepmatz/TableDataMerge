use clap::Parser;
use std::ffi::OsStr;
use std::path::Path;
use std::fs;

use crate::error::TdmError;
use cli::{Args, OutTypes};
use table::Table;

use crate::table::FormatOptions;

mod cli;
mod error;
pub mod table;
pub mod util;

fn main() {
    let args = Args::parse();

    let mut tables: Vec<(String, Table)> = Vec::with_capacity(args.files.len());
    let file_contents = args.files.iter().map(|f| {
        (
            f,
            fs::read_to_string(&f.0).unwrap_or_else(|error| {
                TdmError::ReadFile {
                    path: f.0.clone(),
                    error,
                }
                .handle();
            }),
        )
    });
    for ((file, additional_data), content) in file_contents {
        let file_path = Path::new(file);
        let file_stem = file_path
            .file_stem()
            .and_then(OsStr::to_str)
            .unwrap_or_else(|| {
                TdmError::DetermineFileStem {
                    file: file.to_owned(),
                }
                .handle()
            });
        let file_type = file_path.extension().and_then(OsStr::to_str);
        let parse_res = match file_type {
            Some("txt" | "dat") => Table::from_dat(&content, additional_data),
            Some("json") => Table::from_json(&content, additional_data),
            Some("csv") => Table::from_csv(&content, additional_data),
            Some("m") => Table::from_m(&content, additional_data),
            Some(file_type) => TdmError::UnknownFileType {
                file_type: file_type.to_owned(),
            }
            .handle(),
            None => TdmError::DetermineFileType {
                file: file.to_owned(),
            }
            .handle(),
        };
        match parse_res {
            Ok(table) => tables.push((file_stem.to_string(), table)),
            Err(error) => TdmError::ParseTable(error).handle(),
        }
    }

    let out_path = match &args.out {
        None => {
            let mut basename = itertools::join(tables.iter().map(|i| i.0.clone()), "_");
            match &args.to {
                OutTypes::Csv => basename += ".csv",
                OutTypes::Dat => basename += ".dat",
                OutTypes::Tex => basename += ".tex",
                OutTypes::Md => basename += ".md",
                OutTypes::Json => basename += ".json",
            }
            String::from("tdm_") + basename.as_str()
        }
        Some(arg) => arg.to_string(),
    };
    let out_path = Path::new(&out_path);

    let format_options = FormatOptions::from(args.clone());

    let mut tables = tables.into_iter().map(|(_, table)| table);
    let mut first_table = tables.next().unwrap(); // infallible

    match args.vertical {
        false => {
            for table in tables {
                first_table.concat(table);
            }
        }
        true => {
            for table in tables {
                first_table.stack(table);
            }
        }
    }

    let output = match args.to {
        OutTypes::Csv => first_table.to_csv(&format_options),
        OutTypes::Dat => first_table.to_dat(&format_options),
        OutTypes::Tex => first_table.to_tex(&format_options),
        OutTypes::Md => first_table.to_md(&format_options),
        OutTypes::Json => first_table.to_json(&format_options),
    };

    fs::write(out_path, output).unwrap_or_else(|error| {
        TdmError::WriteFile {
            path: out_path
                .to_str()
                .expect("constructed from string")
                .to_owned(),
            error,
        }
        .handle()
    });
}
