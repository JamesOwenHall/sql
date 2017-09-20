use data::Data;

#[derive(Clone, Debug, PartialEq)]
pub struct Answer {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<Data>>,
}
