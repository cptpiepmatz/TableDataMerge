#![allow(non_snake_case)]

use clap::{CommandFactory, Parser, ValueEnum};
use clap::error::ErrorKind;

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
        Args { to: None | Some(OutTypes::Csv | OutTypes::Dat), hline: true, .. } => {
            cmd.error(
                ErrorKind::ArgumentConflict,
                "'--hline' requires '--to' to be 'tex'",
            ).exit();
        }
        Args { to: None | Some(OutTypes::Csv | OutTypes::Dat), math_mode: true, .. } => {
            cmd.error(
                ErrorKind::ArgumentConflict,
                "'--mathmode' requires '--to' to be 'tex'",
            ).exit();
        }
        Args { to: None | Some(OutTypes::Dat | OutTypes::Tex), sep: Some(_), ..} => {
            cmd.error(
                ErrorKind::ArgumentConflict,
                "'--sep' requires '--to' to be 'csv'"
            ).exit();
        }
        _ => {}
    }
}

fn main() {
    let args = Args::parse();
    validate_args(&args);

    dbg!(args);
}
