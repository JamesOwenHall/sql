use std::fmt;
use data::{Data, Number};
use expr::Expr;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum AggregateFunction {
    Average,
    Count,
    Sum,
}

impl AggregateFunction {
    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_ref() {
            "avg" => Some(AggregateFunction::Average),
            "count" => Some(AggregateFunction::Count),
            "sum" => Some(AggregateFunction::Sum),
            _ => None,
        }
    }

    pub fn aggregate(&self) -> Aggregate {
        match self {
            &AggregateFunction::Average => Aggregate::Average(Number::Int(0), 0),
            &AggregateFunction::Count => Aggregate::Count(0),
            &AggregateFunction::Sum => Aggregate::Sum(Number::Int(0)),
        }
    }
}

impl fmt::Display for AggregateFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &AggregateFunction::Average => write!(f, "avg"),
            &AggregateFunction::Count => write!(f, "count"),
            &AggregateFunction::Sum => write!(f, "sum"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Aggregate {
    Average(Number, i64),
    Count(i64),
    Sum(Number),
}

impl Aggregate {
    pub fn apply(&mut self, value: Data) {
        match (self, value) {
            (&mut Aggregate::Average(ref mut acc, ref mut count), Data::Number(ref d)) => {
                *acc += d.clone();
                *count += 1;
            },
            (&mut Aggregate::Count(_), Data::Null) => {},
            (&mut Aggregate::Count(ref mut acc), _) => *acc += 1,
            (&mut Aggregate::Sum(ref mut acc), Data::Number(ref n)) => *acc += n.clone(),
            _ => {},
        }
    }

    pub fn final_value(&self) -> Data {
        match self {
            &Aggregate::Average(_, 0) => Data::Number(Number::Float(0.0)),
            &Aggregate::Average(ref acc, ref count) => Data::Number(Number::Float(acc.as_float() / (*count as f64))),
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

    #[test]
    fn average_nothing() {
        let input = data_vec![];
        let expected = Data::Number(Number::Float(0.0));
        assert_eq!(expected, apply_agg("avg", input));
    }

    #[test]
    fn average_something() {
        let input = data_vec![1, 1.5, false];
        let expected = Data::Number(Number::Float(1.25));
        assert_eq!(expected, apply_agg("avg", input));
    }

    fn apply_agg(name: &str, input: Vec<Data>) -> Data {
        let mut agg = AggregateFunction::from_name(name).unwrap().aggregate();
        input.iter().for_each(|value| agg.apply(value.clone()));
        agg.final_value()
    }
}
