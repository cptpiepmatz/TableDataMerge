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

    #[arg(short, long, default_value_t = false, conflicts_with = "dot")]
    pub comma: bool,

    #[arg(short, long, default_value_t = false)]
    pub dot: bool,

    #[arg(short, long, default_value_t = false)]
    pub scientific: bool,

    #[arg(short = 'S', long, default_value_t = false)]
    pub sign: bool,

    #[arg(short, long = "mathmode", default_value_t = false)]
    pub math_mode: bool,

    #[arg(short = 'H', long, default_value_t = false)]
    pub hline: bool,

    #[arg(long)]
    pub sep: Option<String>,

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
}

pub fn validate_args(args: &Args) {
    let mut cmd = Args::command();
    match args {
        Args {
            to: OutTypes::Csv | OutTypes::Dat,
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
            to: OutTypes::Csv | OutTypes::Dat,
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
            to: OutTypes::Dat | OutTypes::Tex,
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
