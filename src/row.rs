use std::collections::HashMap;
use data::Data;
use expr::Expr;

#[derive(Clone, Debug, PartialEq)]
pub struct Row {
    pub fields: HashMap<Expr, Data>,
}

impl Row {
    pub fn new() -> Self {
        Row {
            fields: HashMap::new(),
        }
    }
}

pub fn make_rows(columns: Vec<&'static str>, values: Vec<Vec<Data>>) -> Vec<Row> {
    let mut rows = Vec::new();

    for input_row in values {
        let mut row = Row::new();
        for (index, value) in input_row.into_iter().enumerate() {
            let name = columns[index].to_owned();
            row.fields.insert(Expr::Column(name), value);
        }
        rows.push(row);
    }

    rows
}
