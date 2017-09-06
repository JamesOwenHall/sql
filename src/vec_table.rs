use row::Row;

// #[derive(Clone, Debug, PartialEq)]
pub struct VecTable {
    pub rows: Vec<Row>,
    index: usize,
}

impl VecTable {
    pub fn new() -> Self {
        VecTable{rows: Vec::new(), index: 0}
    }
}

impl Iterator for VecTable {
    type Item = Row;
    fn next(&mut self) -> Option<Self::Item> {
        match self.rows.get(self.index) {
            Some(row) => {
                self.index += 1;
                Some(row.clone())
            }
            None => None,
        }
    }
}
