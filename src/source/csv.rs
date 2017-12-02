extern crate csv;

use std::fs::File;
use data::Data;
use expr::Expr;
use row::Row;
use source::{Source, SourceError};

pub struct CsvSource {
    headers: Vec<String>,
    iter: csv::StringRecordsIntoIter<File>,
}

impl CsvSource {
    pub fn new(filename: &str) -> Result<Source, SourceError> {
        let file = File::open(filename)?;
        let mut reader = csv::Reader::from_reader(file);

        let headers = reader.headers()?
            .iter()
            .map(|header| header.to_owned())
            .collect();

        let records = reader.into_records();
        let source = CsvSource {
            headers: headers,
            iter: records,
        };
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

#[cfg(test)]
mod tests {
    use super::*;
    use row::make_rows;
    use source::open_file;

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
}
