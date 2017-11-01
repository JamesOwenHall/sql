use std::fmt::{self, Display, Formatter};
use aggregate::AggregateCall;
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
            Expr::Column(_) => row.fields.get(self).cloned().unwrap_or(Data::Null),
            Expr::AggregateCall(_) => row.fields.get(self).cloned().unwrap_or(Data::Null),
        }
    }

    pub fn get_aggregate_call(&self) -> Option<AggregateCall> {
        let mut aggregate_call = None;
        self.recurse(|expr: &Expr| {
            if let &Expr::AggregateCall(ref call) = expr {
                aggregate_call = Some(call.clone());
            }
        });
        aggregate_call
    }

    fn recurse<F: FnMut(&Expr)>(&self, mut func: F) {
        match self {
            &Expr::Column(_) => func(self),
            &Expr::AggregateCall(ref call) => {
                func(self);
                call.argument.recurse(func);
            }
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            &Expr::Column(ref name) => write!(f, r#""{}""#, name),
            &Expr::AggregateCall(ref call) => write!(f, "{}({})", call.function, call.argument)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use data::Number;

    #[test]
    fn eval_column() {
        let mut row = Row::new();
        row.fields.insert(Expr::Column(String::from("a")), Data::Number(Number::Int(0)));
        row.fields.insert(Expr::Column(String::from("b")), Data::Number(Number::Int(1)));
        row.fields.insert(Expr::Column(String::from("c")), Data::Number(Number::Int(2)));

        let expr = Expr::Column(String::from("b"));
        assert_eq!(Data::Number(Number::Int(1)), expr.eval(&row));
    }
}
