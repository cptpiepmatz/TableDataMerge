use std::fmt::{Display, Formatter};
use clap::error::ErrorKind;
use clap::{CommandFactory, Parser, ValueEnum};

#[derive(Parser, Debug, Clone)]
pub struct Args {
    #[arg(short, long)]
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

    #[arg(short, long = "mathmode", default_value_t = false)]
    pub math_mode: bool,

    #[arg(short = 'H', long, default_value_t = false)]
    pub hline: bool,

    #[arg(long, long, default_value = ",")]
    pub csv_sep: String,

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
    Json
}

#[derive(ValueEnum, Debug, Copy, Clone, Default)]
pub enum DecimalSeparator {
    #[default]
    Dot,
    Comma
}
