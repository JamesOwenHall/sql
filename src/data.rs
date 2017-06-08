#[derive(Clone, Debug, PartialEq)]
pub enum Data {
    Null,
    Bool(bool),
    Int(i64),
    String(String),
}
