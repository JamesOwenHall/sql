use std::error::Error;
use std::fs::File;
use data::Data;
use expr::Expr;
use row::Row;
use csv;

pub type Source = Box<Iterator<Item=Result<Row, SourceError>>>;

#[derive(Clone, Debug)]
pub struct SourceError {
    description: String
}

impl<E: Error> From<E> for SourceError {
    fn from(e: E) -> SourceError {
        SourceError{description: e.description().to_owned()}
    }
}

pub fn open_file(filename: &str) -> Result<Source, SourceError> {
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

pub struct CsvSource {
    headers: Vec<String>,
    iter: csv::StringRecordsIntoIter<File>,
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
