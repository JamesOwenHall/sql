#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub enum Data {
    Null,
    Bool(bool),
    Int(i64),
    String(String),
}
