use data::Data;
use row::Row;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Expr {
    Column(String),
    AggregateCall(AggregateCall),
}

impl Expr {
    pub fn eval(&self, row: &Row) -> Data {
        match *self {
            Expr::Column(ref name) => row.field_value(name).unwrap_or(Data::Null),
            Expr::AggregateCall(ref call) => row.aggregate_value(call).unwrap_or(Data::Null),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AggregateCall {
    pub function: String,
    pub argument: Box<Expr>,
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
