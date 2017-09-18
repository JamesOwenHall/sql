use std::fmt;
use data::Data;
use expr::Expr;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum AggregateFunction {
    Sum,
}

impl AggregateFunction {
    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_ref() {
            "sum" => Some(AggregateFunction::Sum),
            _ => None,
        }
    }

    pub fn aggregate(&self) -> Aggregate {
        match self {
            &AggregateFunction::Sum => Aggregate::Sum(0)
        }
    }
}

impl fmt::Display for AggregateFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &AggregateFunction::Sum => write!(f, "sum")
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Aggregate {
    Sum(i64),
}

impl Aggregate {
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
    pub function: AggregateFunction,
    pub argument: Box<Expr>,
}
