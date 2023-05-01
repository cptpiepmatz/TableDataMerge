use clap::Parser;
use std::ffi::OsStr;
use std::path::Path;
use std::{fs, process};

use cli::{Args, OutTypes};
use table::Table;

use crate::table::FormatOptions;

mod cli;
mod table;
mod util;

fn main() {
    let args = Args::parse();

    let mut tables: Vec<(String, Table)> = Vec::with_capacity(args.files.len());
    let file_contents = args.files.iter().map(|f| {
        (
            f,
            fs::read_to_string(f).unwrap_or_else(|e| {
                eprintln!("{e}");
                process::exit(1);
            }),
        )
    });
    for (file, content) in file_contents {
        let file_path = Path::new(file);
        let file_stem = file_path
            .file_stem()
            .and_then(OsStr::to_str)
            .unwrap_or_else(|| {
                eprintln!("could not determine file stem for '{file}'");
                process::exit(1);
            });
        let file_type = file_path.extension().and_then(OsStr::to_str);
        let parse_res = match file_type {
            Some("txt" | "dat") => Table::from_dat(&content),
            Some("json") => Table::from_json(&content),
            Some(file_type) => {
                eprintln!("unknown file type '{file_type}'");
                process::exit(1);
            }
            None => {
                eprintln!("could not determine file type for file '{file}'");
                process::exit(1);
            }
        };
        match parse_res {
            Ok(table) => tables.push((file_stem.to_string(), table)),
            Err(_) => {
                eprintln!("could not parse table '{file}'");
                process::exit(1);
            }
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
        OutTypes::Csv => todo!(),
        OutTypes::Dat => first_table.to_dat(&format_options),
        OutTypes::Tex => first_table.to_tex(&format_options),
        OutTypes::Md => first_table.to_md(&format_options),
        OutTypes::Json => first_table.to_json(&format_options),
    };

    fs::write(out_path, output).unwrap();
}
