use crate::table::cell::Cell;
use crate::util::{AnyRange, ParseAnyRangeError};
use clap::error::ErrorKind;
use clap::{CommandFactory, Parser, ValueEnum};
use std::convert::Infallible;
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

    // files with additional data
    #[arg(required = true, num_args(1..), value_parser = parse_file_path)]
    pub files: Vec<(String, Option<String>)>,
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

/// Represents an error that occurs when parsing a fix (prefix or suffix).
#[derive(Debug)]
enum ParseFixError {
    InvalidRange(ParseAnyRangeError)
}

impl Display for ParseFixError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseFixError::InvalidRange(r) => Display::fmt(r, f)
        }
    }
}

impl From<ParseAnyRangeError> for ParseFixError {
    fn from(value: ParseAnyRangeError) -> Self {
        ParseFixError::InvalidRange(value)
    }
}

impl Error for ParseFixError {}

/// Parses a fix (prefix or suffix) provided as a command-line argument.
/// The input should be in the format "<range>:<fix>".
/// The range is a typical Rust range, and the fix is a string to be placed at the specified range.
fn parse_fix(input: &str) -> Result<(AnyRange<usize>, String), ParseFixError> {
    let mut split = input.splitn(2, ':');

    let range = split.next().expect("first always exists");
    let range: AnyRange<usize> = range.parse()?;

    let fix = split.next().unwrap_or("");

    Ok((range, fix.to_string()))
}

/// Parses the input file path, which may include additional data appended with a ":".
/// Since ":" is not a valid character for file paths in most relevant file systems, it can be
/// safely used as a separator here.
/// If no ":" is found, this function returns the file path and a `None`.
fn parse_file_path(input: &str) -> Result<(String, Option<String>), Infallible> {
    let mut split = input.splitn(2, ':');
    let file_path = split.next().expect("first always exists").to_string();
    let additional_data = split.next().map(str::to_string);
    Ok((file_path, additional_data))
}
