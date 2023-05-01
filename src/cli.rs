use crate::table::cell::Cell;
use crate::util::AnyRange;
use clap::error::ErrorKind;
use clap::{CommandFactory, Parser, ValueEnum};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Parser, Debug, Clone)]
pub struct Args {
    #[arg()]
    pub to: OutTypes,

    #[arg(short, long)]
    pub out: Option<String>,

    #[arg(short, long)]
    pub precision: Option<u16>,

    #[arg(short, long, default_value = "dot")]
    pub decimal_sep: DecimalSeparator,

    #[arg(short, long = "exponent", default_value_t = false)]
    pub exponent: bool,

    #[arg(short, long, default_value_t = false)]
    pub sign: bool,

    #[arg(short = 'H', long, default_value_t = false)]
    pub hline: bool,

    #[arg(short, long, default_value = ",")]
    pub csv_sep: String,

    #[arg(short = 'P', long, num_args(1), value_parser = parse_fix)]
    pub prefix: Vec<(AnyRange<usize>, String)>,

    #[arg(short = 'S', long, num_args(1), value_parser = parse_fix)]
    pub suffix: Vec<(AnyRange<usize>, String)>,

    #[arg(short, long, default_value_t = false)]
    pub vertical: bool,

    #[arg(required = true, num_args(1..))]
    pub files: Vec<String>,
}

#[derive(ValueEnum, Debug, Copy, Clone)]
pub enum OutTypes {
    Csv,
    Dat,
    Tex,
    Md,
    Json,
}

#[derive(ValueEnum, Debug, Copy, Clone, Default)]
pub enum DecimalSeparator {
    #[default]
    Dot,
    Comma,
}

#[derive(Debug)]
struct ParseFixError;

impl Display for ParseFixError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for ParseFixError {}

fn parse_fix(input: &str) -> Result<(AnyRange<usize>, String), ParseFixError> {
    let mut split = input.splitn(2, ':');

    // TODO: handle this better
    let range = split.next().unwrap();
    let range: AnyRange<usize> = range.parse().unwrap();

    let fix = split.next().unwrap();

    Ok((range, fix.to_string()))
}
