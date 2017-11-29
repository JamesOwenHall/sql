use std::error::Error;
use std::fs::File;
use std::path::Path;
use data::{Data, Number};
use expr::Expr;
use row::Row;
use csv;
use serde_json;

pub type Source = Box<Iterator<Item = Result<Row, SourceError>>>;

#[derive(Clone, Debug, PartialEq)]
pub struct SourceError {
    pub description: String,
}

impl<E: Error> From<E> for SourceError {
    fn from(e: E) -> SourceError {
        SourceError{description: e.description().to_owned()}
    }
}

pub fn open_file(filename: &str) -> Result<Source, SourceError> {
    let path = Path::new(filename);
    match path.extension().and_then(|s| s.to_str()) {
        Some("csv") => CsvSource::new(&filename),
        Some("json") => JsonSource::new(&filename),
        Some(e) => Err(SourceError{description: format!("unknown file extension: .{}", e)}),
        None => Err(SourceError{description: "unknown file type".to_owned()}),
    }
}

pub struct CsvSource {
    headers: Vec<String>,
    iter: csv::StringRecordsIntoIter<File>,
}

impl CsvSource {
    fn new(filename: &str) -> Result<Source, SourceError> {
        let file = File::open(filename)?;
        let mut reader = csv::Reader::from_reader(file);

        let headers = reader.headers()?
            .iter()
            .map(|header| header.to_owned())
            .collect();

        let records = reader.into_records();
        let source = CsvSource{headers: headers, iter: records};
        Ok(Box::new(source))
    }
}

impl Iterator for CsvSource {
    type Item = Result<Row, SourceError>;
    fn next(&mut self) -> Option<Self::Item> {
        let record = match self.iter.next() {
            None => return None,
            Some(Err(e)) => return Some(Err(e.into())),
            Some(Ok(rec)) => rec,
        };

        let mut row = Row::new();
        for (index, field) in record.into_iter().enumerate() {
            let column = Expr::Column(self.headers[index].clone());
            row.fields.insert(column, Data::String(field.to_owned()));
        }

        Some(Ok(row))
    }
}

pub struct JsonSource {
    values: ::std::vec::IntoIter<serde_json::Value>,
}

impl JsonSource {
    fn new(filename: &str) -> Result<Source, SourceError> {
        let file = File::open(filename)?;
        let data: serde_json::Value = serde_json::from_reader(file)?;
        let rows = match data {
            serde_json::Value::Array(a) => a,
            _ => return Err(SourceError{
                description: "invalid JSON".to_owned(),
            }),
        };

        Ok(Box::new(JsonSource{values: rows.into_iter()}))
    }
}

impl Iterator for JsonSource {
    type Item = Result<Row, SourceError>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let map = match self.values.next() {
                None => return None,
                Some(serde_json::Value::Object(m)) => m,
                Some(_) => continue,
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
                    },
                    serde_json::Value::String(s) => Data::String(s),
                    _ => continue,
                };

                row.fields.insert(Expr::Column(key), val);
            }
            return Some(Ok(row))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use row::make_rows;

    #[test]
    fn csv_source() {
        let source = open_file("fixtures/accounts.csv").unwrap();
        let expected =
            make_rows(
                vec!["id", "name", "balance", "frozen", "last_transaction_amount"],
                vec![
                    data_vec!["1000", "Alice", "15.50", "false", "-4.50"],
                    data_vec!["1001", "Bob", "-50.08", "true", "-100.99"],
                    data_vec!["1002", "Charlie", "0.00", "false", ""],
                    data_vec!["1003", "Denise", "-1024.64", "true", "-1024.64"],
                ],
            );
        let actual: Vec<Result<Row, SourceError>> = source.collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn json_source() {
        let source = open_file("fixtures/accounts.json").unwrap();
        let expected = make_rows(
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
