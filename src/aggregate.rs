use data::Data;
use expr::Expr;

#[derive(Clone, Debug, PartialEq)]
pub enum Aggregate {
    Sum(i64),
}

impl Aggregate {
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "sum" => Some(Aggregate::Sum(0)),
            _ => None,
        }
    }

    pub fn apply(&mut self, value: Data) {
        match (self, value) {
            (&mut Aggregate::Sum(ref mut acc), Data::Int(i)) => *acc += i,
            _ => {},
        }
    }

    pub fn final_value(&self) -> Data {
        match self {
            &Aggregate::Sum(acc) => Data::Int(acc),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AggregateCall {
    pub function: String,
    pub argument: Box<Expr>,
}
