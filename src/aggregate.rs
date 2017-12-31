use std::fmt;
use data::{Data, Number};
use expr::Expr;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum AggregateFunction {
    Count,
    Sum,
}

impl AggregateFunction {
    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_ref() {
            "count" => Some(AggregateFunction::Count),
            "sum" => Some(AggregateFunction::Sum),
            _ => None,
        }
    }

    pub fn aggregate(&self) -> Aggregate {
        match self {
            &AggregateFunction::Count => Aggregate::Count(0),
            &AggregateFunction::Sum => Aggregate::Sum(Number::Int(0)),
        }
    }
}

impl fmt::Display for AggregateFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &AggregateFunction::Count => write!(f, "count"),
            &AggregateFunction::Sum => write!(f, "sum"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Aggregate {
    Count(i64),
    Sum(Number),
}

impl Aggregate {
    pub fn apply(&mut self, value: Data) {
        match (self, value) {
            (&mut Aggregate::Count(_), Data::Null) => {},
            (&mut Aggregate::Count(ref mut acc), _) => *acc += 1,
            (&mut Aggregate::Sum(ref mut acc), Data::Number(ref n)) => *acc += n.clone(),
            _ => {},
        }
    }

    pub fn final_value(&self) -> Data {
        match self {
            &Aggregate::Count(ref acc) => Data::Number(Number::Int(acc.clone())),
            &Aggregate::Sum(ref acc) => Data::Number(acc.clone()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AggregateCall {
    pub function: AggregateFunction,
    pub argument: Box<Expr>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_ints() {
        let input = data_vec![0, 1, 2, 3];
        let expected = Data::Number(Number::Int(6));
        assert_eq!(expected, apply_agg("sum", input));
    }

    #[test]
    fn sum_floats() {
        let input = data_vec![0.0, -1.2, 2.4, 3.6];
        let expected = Data::Number(Number::Float(4.8));
        assert_eq!(expected, apply_agg("sum", input));
    }

    #[test]
    fn sum_mixed() {
        let input = data_vec![Data::Null, 1, true, false, 2.0, "foo"];
        let expected = Data::Number(Number::Float(3.0));
        assert_eq!(expected, apply_agg("sum", input));
    }

    #[test]
    fn count() {
        let input = data_vec![Data::Null, 1, true, false, 2.0, "foo"];
        let expected = Data::Number(Number::Int(5));
        assert_eq!(expected, apply_agg("count", input));
    }

    fn apply_agg(name: &str, input: Vec<Data>) -> Data {
        let mut agg = AggregateFunction::from_name(name).unwrap().aggregate();
        input.iter().for_each(|value| agg.apply(value.clone()));
        agg.final_value()
    }
}
