use std::fmt::{self, Display, Formatter};
use aggregate::AggregateCall;
use data::Data;
use row::Row;
use scanner::Token;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Expr {
    Column(String),
    AggregateCall(AggregateCall),
    BinaryExpr {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
}

impl Expr {
    pub fn eval(&self, row: &Row) -> Data {
        match *self {
            Expr::Column(_) => row.fields.get(self).cloned().unwrap_or(Data::Null),
            Expr::AggregateCall(_) => row.fields.get(self).cloned().unwrap_or(Data::Null),
            Expr::BinaryExpr{ref left, ref op, ref right} => op.eval(left.eval(row), right.eval(row)),
        }
    }

    pub fn get_aggregate_call(&self) -> Option<AggregateCall> {
        let mut aggregate_call = None;
        self.recurse(&mut |expr: &Expr| {
            if let &Expr::AggregateCall(ref call) = expr {
                aggregate_call = Some(call.clone());
            }
        });
        aggregate_call
    }

    fn recurse<F: FnMut(&Expr)>(&self, func: &mut F) {
        match self {
            &Expr::Column(_) => func(self),
            &Expr::AggregateCall(ref call) => {
                func(self);
                call.argument.recurse(func);
            },
            &Expr::BinaryExpr{ref left, op: _, ref right} => {
                left.recurse(func);
                right.recurse(func);
            },
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            &Expr::Column(ref name) => Token::Identifier(name.clone()).fmt(f),
            &Expr::AggregateCall(ref call) => write!(f, "{}({})", call.function, call.argument),
            &Expr::BinaryExpr{ref left, ref op, ref right} => write!(f, "{} {} {}", left, op, right),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BinaryOp {
    Eq,
}

impl BinaryOp {
    pub fn eval(&self, left: Data, right: Data) -> Data {
        match self {
            &BinaryOp::Eq => Data::Bool(left == right),
        }
    }
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &BinaryOp::Eq => write!(f, "="),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aggregate::{AggregateCall, AggregateFunction};
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

    #[test]
    fn eval_aggregate_function() {
        let agg_call = AggregateCall {
            function: AggregateFunction::Sum,
            argument: Box::new(Expr::Column(String::from("a"))),
        };

        let mut row = Row::new();
        row.fields.insert(Expr::AggregateCall(agg_call.clone()), Data::Number(Number::Int(4)));

        let expr = Expr::AggregateCall(agg_call);
        assert_eq!(Data::Number(Number::Int(4)), expr.eval(&row));
    }

    #[test]
    fn eval_binary_op() {
        let cases = vec![
            (BinaryOp::Eq, Data::Bool(false), Data::Bool(false), Data::Bool(true)),
            (BinaryOp::Eq, Data::Bool(false), Data::Bool(true), Data::Bool(false)),
            (BinaryOp::Eq, Data::Null, Data::Null, Data::Bool(true)),
            (BinaryOp::Eq, Data::String(String::from("foo")), Data::Null, Data::Bool(false)),
        ];

        for (op, left, right, expected) in cases {
            assert_eq!(expected, op.eval(left, right));
        }
    }
}
