use data::Data;
use row::Row;

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Column(String),
}

impl Expr {
    pub fn eval(&self, row: &Row) -> Data {
        match *self {
            Expr::Column(ref name) => row.value(name).unwrap_or(Data::Null),
        }
    }
}
