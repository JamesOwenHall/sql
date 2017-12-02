mod csv;
mod json;

use std::error::Error;
use std::path::Path;
pub use self::csv::CsvSource;
pub use self::json::JsonSource;
use row::Row;

pub type Source = Box<Iterator<Item = Result<Row, SourceError>>>;

#[derive(Clone, Debug, PartialEq)]
pub struct SourceError {
    pub description: String,
}

impl<E: Error> From<E> for SourceError {
    fn from(e: E) -> SourceError {
        SourceError { description: e.description().to_owned() }
    }
}

pub fn open_file(filename: &str) -> Result<Source, SourceError> {
    let path = Path::new(filename);
    match path.extension().and_then(|s| s.to_str()) {
        Some("csv") => CsvSource::new(&filename),
        Some("json") => JsonSource::new(&filename),
        Some(e) => Err(SourceError { description: format!("unknown file extension: .{}", e) }),
        None => Err(SourceError { description: "unknown file type".to_owned() }),
    }
}
