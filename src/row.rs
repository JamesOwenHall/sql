use std::collections::HashMap;
use data::Data;
use expr::Expr;
use source::SourceError;

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

pub fn make_rows(columns: Vec<&'static str>, values: Vec<Vec<Data>>) -> Vec<Result<Row, SourceError>> {
    let mut rows = Vec::new();

    for input_row in values {
        let mut row = Row::new();
        for (index, value) in input_row.into_iter().enumerate() {
            let name = columns[index].to_owned();
            row.fields.insert(Expr::Column(name), value);
        }
        rows.push(Ok(row));
    }

    rows
}

#[macro_export]
macro_rules! row {
    ( $( $x:expr ),* ) => ( vec![ $( Data::from($x) ),* ] )
}
