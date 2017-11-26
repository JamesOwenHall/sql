use std::fmt;
use data::Data;
use query::SortDirection;

#[derive(Clone, Debug, PartialEq)]
pub struct Answer {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<Data>>,
}

impl Answer {
    pub fn sort(&mut self, column_indices: &[(usize, SortDirection)]) {
        for &(index, ref direction) in column_indices.iter().rev() {
            match direction {
                &SortDirection::Asc => self.rows.sort_by(|a, b| a[index].cmp(&b[index])),
                &SortDirection::Desc => self.rows.sort_by(|a, b| b[index].cmp(&a[index])),
            }
        }
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
