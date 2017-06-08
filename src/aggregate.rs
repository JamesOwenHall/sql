use data::Data;
use expr::Expr;
use row::Row;

#[derive(Clone, Debug, PartialEq)]
pub enum Aggregate {
    Sum(i64),
    Avg(i64, u64),
}

impl Aggregate {
    pub fn apply(&mut self, data: Data) {
        match (self, data) {
            (&mut Aggregate::Sum(ref mut n), Data::Int(i)) => *n += i,
            (&mut Aggregate::Avg(ref mut n, ref mut count), Data::Int(i)) => {
                *n += i;
                *count += 1;
            },
            _ => {},
        }
    }

    pub fn value(&self) -> Data {
        match self {
            &Aggregate::Sum(n) => Data::Int(n),
            &Aggregate::Avg(n, count) => Data::Int(n / count as i64),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AggregateCall {
    pub aggregate: Aggregate,
    pub expr: Expr,
}

impl AggregateCall {
    pub fn apply(&mut self, row: &Row) {
        self.aggregate.apply(self.expr.eval(row));
    }
}
