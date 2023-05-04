use crate::table::cell::Cell;
use crate::util::{AnyRange, ParseAnyRangeError};
use clap::error::ErrorKind;
use clap::{CommandFactory, Parser, ValueEnum};
use std::convert::Infallible;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

/// TableDataMerge
///
/// 'tdm' is a versatile command-line tool for merging plain-text tables horizontally or vertically.
/// It provides support for various input and output data types, and offers cell formatting options,
/// including number formatting, prefixes, and suffixes.
#[derive(Parser, Debug, Clone)]
pub struct Args {
    /// Output data type
    ///
    /// Specifies the output data type.
    #[arg()]
    pub to: OutTypes,

    /// Output file path
    ///
    /// Sets the output file path.
    /// If not provided, the output path will be generated from the input file stems and the output data type.
    #[arg(short, long)]
    pub out: Option<String>,

    /// Amount of decimal places
    ///
    /// Sets the number of decimal places for numerical values, which will be correctly rounded.
    #[arg(short, long)]
    pub precision: Option<u16>,

    /// Decimal separator
    ///
    /// Sets the decimal separator for numerical values.
    #[arg(short, long, default_value = "dot")]
    pub decimal_sep: DecimalSeparator,

    /// Scientific notation
    ///
    /// Enables scientific notation for numerical cells (e.g., '1.234e+05').
    #[arg(short, long = "exponent", default_value_t = false)]
    pub exponent: bool,

    /// Sign every number
    ///
    /// Forces a sign on every number.
    /// By default, only negative values have a sign. This setting forces positive values to have a "+" prefix.
    #[arg(short, long, default_value_t = false)]
    pub sign: bool,

    /// Inject '\hline'
    ///
    /// Inserts '\hline' between lines when using 'tex' output format.
    #[arg(short = 'H', long, default_value_t = false)]
    pub hline: bool,

    /// Choose csv delimiter
    ///
    /// Specifies the value separator for 'csv' output format.
    #[arg(short, long, default_value = ",")]
    pub csv_sep: String,

    /// Numerical prefixes
    ///
    /// Sets prefixes for numerical cells.
    /// Use multiple times to set different prefixes for different ranges.
    /// Argument format: '<range>:<fix>', where 'fix' is the string placed before the cell.
    /// Ranges follow Rust syntax (e.g., '1..2', '..2', '1..', '..' or '1') and are 0-indexed.
    /// Overlapping ranges will override previous rules.
    /// To use spaces consider the syntax: '-P "1..2: m"'.
    #[arg(short = 'P', long, num_args(1), value_parser = parse_fix)]
    pub prefix: Vec<(AnyRange<usize>, String)>,

    /// Numerical suffixes
    ///
    /// Sets suffixes for numerical cells.
    /// Follows the same format and behavior as the `prefix` option.
    #[arg(short = 'S', long, num_args(1), value_parser = parse_fix)]
    pub suffix: Vec<(AnyRange<usize>, String)>,

    /// Stack tables
    ///
    /// Stacks tables vertically instead of concatenating them horizontally.
    #[arg(short, long, default_value_t = false)]
    pub vertical: bool,

    /// File paths
    ///
    /// Specifies input file paths and optional additional data.
    /// Format: '<file_path>:<additional_data>'.
    /// For .csv files, additional data can set a custom delimiter (e.g., ';' or '|').
    /// Example: 'example.csv:;'.
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
