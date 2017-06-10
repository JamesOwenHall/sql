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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn eval_column() {
        let row = {
            let mut fields = HashMap::new();
            fields.insert(String::from("a"), Data::Int(0));
            fields.insert(String::from("b"), Data::Int(1));
            fields.insert(String::from("c"), Data::Int(2));
            Row::with_fields(fields)
        };

        let expr = Expr::Column(String::from("b"));
        assert_eq!(Data::Int(1), expr.eval(&row));
    }
}
