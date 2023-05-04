use crate::cli::DecimalSeparator;
use crate::table::FormatOptions;
use format_num::NumberFormat;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

/// A cell that can hold integer, float, or string values, or be blank.
///
/// The `Cell` enum represents a single element in a table.
/// The numerical options allow for more precise formatting of these values.
#[derive(Default, Clone)]
pub enum Cell {
    Int(i32),
    Float(f64),
    Str(String),

    #[default]
    Blank,
}

impl Cell {
    /// Formats the cell content as a string to be used in a table.
    ///
    /// The method uses the provided format options from the command-line interface.
    /// The index is used to check if some cells should get custom formatting.
    ///
    /// # Arguments
    ///
    /// * `format_options` - A reference to the format options specified by the user.
    /// * `index` - The index of the cell in the table.
    ///
    /// # Returns
    ///
    /// A `String` containing the formatted cell content.
    pub fn fmt(&self, format_options: &FormatOptions, index: usize) -> String {
        // The prefix and suffix are selected from the format options, checking if the index is
        // within one of the rules.
        // Later rules can potentially override previously evaluated rules.

        // Prepare the prefix by iterating through the format options' prefix rules.
        // Most cells are expected to be numerical, so there's no need to test for cell type here.
        let mut prefix = "";
        for (range, prefix_str) in format_options.prefix.iter() {
            if range.contains(&index) {
                prefix = prefix_str;
            }
        }

        // Prepare the suffix by iterating through the format options' suffix rules.
        // Most cells are expected to be numerical, so there's no need to test for cell type here.
        let mut suffix = "";
        for (range, suffix_str) in format_options.suffix.iter() {
            if range.contains(&index) {
                suffix = suffix_str;
            }
        }

        // Format the cell content based on its type, adding the prefix and suffix as necessary.
        match self {
            Cell::Int(v) => {
                prefix.to_string() + Cell::fmt_num(*v, format_options).as_str() + suffix
            }
            Cell::Float(v) => {
                prefix.to_string() + Cell::fmt_num(*v, format_options).as_str() + suffix
            }
            Cell::Str(s) => s.to_owned(),
            Cell::Blank => String::from(""),
        }
    }

    /// Formats numerical cell values with the given format options.
    ///
    /// # Arguments
    ///
    /// * `value` - The numerical value of the cell.
    /// * `format_options` - A reference to the format options specified by the user.
    ///
    /// # Returns
    ///
    /// A `String` containing the formatted numerical cell value.
    fn fmt_num<T>(value: T, format_options: &FormatOptions) -> String
    where
        T: Into<f64> + Display + PartialOrd + Copy,
    {
        let nf = NumberFormat::new();
        let formatted = match (
            format_options.precision,
            format_options.exponent,
            format_options.sign,
        ) {
            (None, false, true) if value.into() >= 0.0 => format!("+{value}"),
            (None, false, _) => value.to_string(),
            (None, true, false) => nf.format("e", value),
            (None, true, true) => nf.format("+e", value),
            (Some(p), false, false) => nf.format(format!("0.{p}f").as_str(), value),
            (Some(p), false, true) => nf.format(format!("+0.{p}f").as_str(), value),
            (Some(p), true, false) => nf.format(format!(".{p}e").as_str(), value),
            (Some(p), true, true) => nf.format(format!("+.{p}e").as_str(), value),
        };

        match format_options.decimal_sep {
            DecimalSeparator::Dot => formatted,
            DecimalSeparator::Comma => formatted.replace(".", ","),
        }
    }
}

/// Implementation of the `FromStr` trait for cells.
///
/// This infallible conversion tries to parse numerical values in cells, first attempting to parse
/// integers, then floats
/// (first using the default "." decimal separator, then with the "," replaced with ".").
/// If the cell only consists of trimmable characters, the cell is considered blank.
/// Otherwise, the cell content is stored as a string.
impl FromStr for Cell {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(int) = s.parse::<i32>() {
            return Ok(Cell::Int(int));
        }

        if let Ok(float) = s.parse::<f64>() {
            return Ok(Cell::Float(float));
        }

        if let Ok(float) = s.replace(',', ".").parse::<f64>() {
            return Ok(Cell::Float(float));
        }

        if s.trim().chars().count() == 0 {
            return Ok(Cell::Blank);
        }

        Ok(Cell::Str(s.to_string()))
    }
}

/// Custom implementation of the `Debug` trait for `Cell`.
///
/// This implementation provides a simpler representation of the cell content and its type.
impl Debug for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(v) => write!(f, "{v}i"),
            Self::Float(v) => write!(f, "{v}f"),
            Self::Str(v) => write!(f, "{v}"),
            Self::Blank => write!(f, "[]"),
        }
    }
}
