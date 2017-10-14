use std::fs::File;
use std::io;
use data::Data;
use expr::Expr;
use row::Row;
use csv;

pub fn open_file(filename: &str) -> io::Result<Box<Iterator<Item=Row>>> {
    let file = File::open(filename)?;
    let mut reader = csv::Reader::from_reader(file);
    
    let headers = reader.headers()
        .unwrap()
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
    type Item = Row;
    fn next(&mut self) -> Option<Self::Item> {
        let record = match self.iter.next() {
            None => return None,
            Some(rec) => rec.unwrap(),
        };

        let mut row = Row::new();
        for (index, field) in record.into_iter().enumerate() {
            let column = Expr::Column(self.headers[index].clone());
            row.fields.insert(column, Data::String(field.to_owned()));
        }

        Some(row)
    }
}
