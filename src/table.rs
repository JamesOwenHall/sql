use std::fmt::Debug;
use row::Row;

pub trait Table: Debug {
    fn for_each(&self, func: &mut FnMut(&Row));
}

#[derive(Clone, Debug, PartialEq)]
pub struct VecTable {
    pub rows: Vec<Row>,
}

impl VecTable {
    pub fn new() -> Self {
        VecTable{rows: Vec::new()}
    }
}

impl Table for VecTable {
    fn for_each(&self, func: &mut FnMut(&Row)) {
        for row in self.rows.iter() {
            func(row);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;
    use data::Data;

    #[test]
    fn vec_table_for_each() {
        let mut table = VecTable::new();
        table.rows.push({
            let mut fields = HashMap::new();
            fields.insert(String::from("a"), Data::Int(0));
            fields.insert(String::from("b"), Data::Int(1));
            fields.insert(String::from("c"), Data::Int(2));
            Row::with_fields(fields)
        });
        table.rows.push({
            let mut fields = HashMap::new();
            fields.insert(String::from("a"), Data::Int(3));
            fields.insert(String::from("b"), Data::Int(4));
            fields.insert(String::from("c"), Data::Int(5));
            Row::with_fields(fields)
        });

        let mut actual = Vec::new();
        table.for_each(&mut |row| actual.push(row.clone()));

        assert_eq!(table.rows, actual);
    }
}
