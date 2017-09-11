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
            Expr::Column(ref name) => row.fields.get(name).cloned().unwrap_or(Data::Null),
            Expr::AggregateCall(ref call) => row.aggregates.get(call).cloned().unwrap_or(Data::Null),
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
            &Expr::Column(ref name) => write!(f, "{}", name),
            &Expr::AggregateCall(ref call) => write!(f, "{}({})", call.function, call.argument)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_column() {
        let mut row = Row::new();
        row.fields.insert(String::from("a"), Data::Int(0));
        row.fields.insert(String::from("b"), Data::Int(1));
        row.fields.insert(String::from("c"), Data::Int(2));

        let expr = Expr::Column(String::from("b"));
        assert_eq!(Data::Int(1), expr.eval(&row));
    }
}
