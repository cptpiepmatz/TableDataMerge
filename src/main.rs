#![allow(non_snake_case)]

use std::{fs, process};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use clap::error::ErrorKind;
use clap::{CommandFactory, Parser, ValueEnum};

use crate::table::Table;

mod table;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    to: Option<OutTypes>,

    #[arg(short, long)]
    out: Option<String>,

    #[arg(short, long)]
    precision: Option<u16>,

    #[arg(short, long, default_value_t = false)]
    comma: bool,

    #[arg(short, long, default_value_t = false)]
    dot: bool,

    #[arg(short, long, default_value_t = false)]
    scientific: bool,

    #[arg(short = 'S', long, default_value_t = false)]
    sign: bool,

    #[arg(short, long = "mathmode", default_value_t = false)]
    math_mode: bool,

    #[arg(short = 'H', long, default_value_t = false)]
    hline: bool,

    #[arg(long)]
    sep: Option<String>,

    #[arg(short, long, default_value_t = false)]
    vertical: bool,

    #[arg(required = true, num_args(1..))]
    files: Vec<String>,
}

#[derive(ValueEnum, Debug, Copy, Clone)]
enum OutTypes {
    Csv,
    Dat,
    Tex,
}

fn validate_args(args: &Args) {
    let mut cmd = Args::command();
    match args {
        Args {
            to: None | Some(OutTypes::Csv | OutTypes::Dat),
            hline: true,
            ..
        } => {
            cmd.error(
                ErrorKind::ArgumentConflict,
                "'--hline' requires '--to' to be 'tex'",
            )
            .exit();
        }
        Args {
            to: None | Some(OutTypes::Csv | OutTypes::Dat),
            math_mode: true,
            ..
        } => {
            cmd.error(
                ErrorKind::ArgumentConflict,
                "'--mathmode' requires '--to' to be 'tex'",
            )
            .exit();
        }
        Args {
            to: None | Some(OutTypes::Dat | OutTypes::Tex),
            sep: Some(_),
            ..
        } => {
            cmd.error(
                ErrorKind::ArgumentConflict,
                "'--sep' requires '--to' to be 'csv'",
            )
            .exit();
        }
        _ => {}
    }
}

fn main() {
    let args = Args::parse();
    validate_args(&args);

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
        let file_stem = file_path.file_stem().and_then(OsStr::to_str).unwrap_or_else(|| {
            eprintln!("could not determine file stem for '{file}'");
            process::exit(1);
        });
        let file_type = file_path.extension().and_then(OsStr::to_str);
        match file_type {
            Some("txt" | "dat") => {
                match Table::from_dat(&content) {
                    Ok(table) => tables.push((file_stem.to_string(), table)),
                    Err(_) => {
                        eprintln!("could not parse table '{file}'");
                        process::exit(1);
                    }
                }
            },
            Some(file_type) => {
                eprintln!("unknown file type '{file_type}'");
                process::exit(1);
            }
            None => {
                eprintln!("could not determine file type for file '{file}'");
                process::exit(1);
            }
        }
    }

    dbg!(tables);

    dbg!(args);
}
