use crate::table::FormatOptions;
use crate::table::Table;

impl Table {
    pub fn to_tex(&self, format_options: &FormatOptions) -> String {
        let mut rows = Vec::with_capacity(self.height);
        for row in self.values.iter() {
            let mut items = Vec::with_capacity(self.width);
            for (i, cell) in row.iter().enumerate() {
                items.push(cell.fmt(format_options, i));
            }
            rows.push(itertools::join(items, " & "));
        }

        match format_options.hline {
            false => itertools::join(rows, " \\\\\n"),
            true => itertools::join(rows, " \\\\\n\\hline\n") + "\\\\\n\\hline\n",
        }
    }
}
