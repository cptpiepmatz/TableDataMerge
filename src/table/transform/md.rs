use crate::table::{FormatOptions, Table};

impl Table {
    pub fn to_md(&self, format_options: &FormatOptions) -> String {
        let mut str_components: Vec<String> = Vec::new();

        // first row
        str_components.push("| ".to_string());
        for (i, cell) in self.values[0].iter().enumerate() {
            str_components.push(cell.fmt(format_options, i));
            str_components.push(String::from(" | "));
        }
        str_components.push(String::from("\n"));

        // header row
        str_components.push(String::from("| "));
        for _ in self.values[0].iter() {
            str_components.push(String::from("--- | "));
        }
        str_components.push(String::from("\n"));

        // rest
        for row in self.values.iter().skip(1) {
            str_components.push(String::from("| "));
            for (i, cell) in row.iter().enumerate() {
                str_components.push(cell.fmt(format_options, i));
                str_components.push(String::from(" | "));
            }
            str_components.push(String::from("\n"));
        }

        str_components.into_iter().collect()
    }
}
