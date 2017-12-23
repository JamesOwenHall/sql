extern crate serde_json;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use data::{Data, Number};
use expr::Expr;
use row::Row;
use source::{Source, SourceError};

pub struct JsonSource {
    lines: Lines<BufReader<File>>,
}

impl JsonSource {
    pub fn new(filename: &str) -> Result<Source, SourceError> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        Ok(Box::new(JsonSource { lines: reader.lines() }))
    }
}

impl Iterator for JsonSource {
    type Item = Result<Row, SourceError>;
    fn next(&mut self) -> Option<Self::Item> {
        let line = match self.lines.next() {
            None => return None,
            Some(Err(e)) => return Some(Err(e.into())),
            Some(Ok(l)) => l,
        };

        let map: HashMap<String, serde_json::Value> = match serde_json::from_str(&line) {
            Ok(map) => map,
            Err(e) => return Some(Err(e.into())),
        };

        let mut row = Row::new();
        for (key, value) in map {
            let val = match value {
                serde_json::Value::Null => Data::Null,
                serde_json::Value::Bool(b) => Data::Bool(b),
                serde_json::Value::Number(n) => {
                    if let Some(i) = n.as_i64() {
                        Data::Number(Number::Int(i))
                    } else {
                        Data::Number(Number::Float(n.as_f64().unwrap()))
                    }
                }
                serde_json::Value::String(s) => Data::String(s),
                _ => continue,
            };

            row.fields.insert(Expr::Column(key), val);
        }
        return Some(Ok(row));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use row::make_rows;
    use source::open_file;

    #[test]
    fn json_source() {
        let source = open_file("fixtures/accounts.json").unwrap();
        let expected =
            make_rows(
                vec!["id", "name", "balance", "frozen", "last_transaction_amount"],
                vec![
                    data_vec![1000, "Alice", 15.5, false, -4.5],
                    data_vec![1001, "Bob", -50.08, true, -100.99],
                    data_vec![1002, "Charlie", 0.0, false, Data::Null],
                    data_vec![1003, "Denise", -1024.64, true, -1024.64],
                ],
            );
        let actual: Vec<Result<Row, SourceError>> = source.collect();
        assert_eq!(expected, actual);
    }
}
