#[derive(Clone, Debug, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub enum Data {
    Null,
    Bool(bool),
    Int(i64),
    String(String),
}
