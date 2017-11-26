use std::fmt;
use data::Data;

#[derive(Clone, Debug, PartialEq)]
pub struct Answer {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<Data>>,
}

impl Answer {
    pub fn sort(&mut self, column_indices: &[usize]) {
        if column_indices.is_empty() {
            return;
        }

        self.rows.sort_unstable_by_key(|row| {
            let mut sort_keys = Vec::with_capacity(column_indices.len());
            for index in column_indices.iter() {
                sort_keys.push(row[*index].clone());
            }
            sort_keys
        });
    }
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.columns.join("\t"))?;

        for row in self.rows.iter() {
            let formatted_cells = row.iter()
                .map(|cell| format!("{}", cell))
                .collect::<Vec<String>>();
            
            writeln!(f, "{}", formatted_cells.join("\t"))?;
        }

        Ok(())
    }
}
