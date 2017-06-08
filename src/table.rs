use std::fmt::Debug;
use row::Row;

pub trait Table: Debug {
    fn for_each(&self, func: &mut FnMut(&Row));
}

#[derive(Clone, Debug, PartialEq)]
pub struct VecTable {
    rows: Vec<Row>,
}

impl Table for VecTable {
    fn for_each(&self, func: &mut FnMut(&Row)) {
        for row in self.rows.iter() {
            func(row);
        }
    }
}
