use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem::transmute;
use std::ops::{Add, AddAssign};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Data {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Data::Null => write!(f, "<null>"),
            &Data::Bool(b) => write!(f, "{}", b),
            &Data::Number(ref n) => write!(f, "{}", n),
            &Data::String(ref s) => write!(f, "{}", s),
        }
    }
}

impl From<bool> for Data {
    fn from(b: bool) -> Self {
        Data::Bool(b)
    }
}

impl From<isize> for Data {
    fn from(i: isize) -> Self {
        Data::Number(Number::Int(i as i64))
    }
}

impl From<f64> for Data {
    fn from(f: f64) -> Self {
        Data::Number(Number::Float(f))
    }
}

impl<'a> From<&'a str> for Data {
    fn from(s: &str) -> Self {
        Data::String(s.to_owned())
    }
}

impl From<String> for Data {
    fn from(s: String) -> Self {
        Data::String(s)
    }
}

#[derive(Clone, Debug, PartialOrd)]
pub enum Number {
    Int(i64),
    Float(f64),
}

impl Number {
    pub fn as_float(&self) -> f64 {
        match self {
            &Number::Int(i) => i as f64,
            &Number::Float(f) => f,
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Number::Int(i) => write!(f, "{}", i),
            &Number::Float(n) => write!(f, "{}", n),
        }
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&Number::Int(i1), &Number::Int(i2)) => i1 == i2,
            (a, b) => equal_floats(a.as_float(), b.as_float()),
        }
    }
}

impl Eq for Number {}
impl Ord for Number {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (&Number::Int(i1), &Number::Int(i2)) => i1.cmp(&i2),
            (a, b) => cmp_floats(a.as_float(), b.as_float()),
        }
    }
}

impl Hash for Number {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let val = match self {
            &Number::Int(i) => i,
            &Number::Float(f) => unsafe{ transmute(f) },
        };
        val.hash(state);
    }
}

impl Add for Number {
    type Output = Number;
    fn add(self, rhs: Number) -> Self::Output {
        match (self, rhs) {
            (Number::Int(i1), Number::Int(i2)) => Number::Int(i1 + i2),
            (a, b) => Number::Float(a.as_float() + b.as_float()),
        }
    }
}

impl AddAssign for Number {
    fn add_assign(&mut self, rhs: Number) {
        match (self.clone(), rhs) {
            (Number::Int(i1), Number::Int(i2)) => *self = Number::Int(i1 + i2),
            (a, b) => *self = Number::Float(a.as_float() + b.as_float()),
        }
    }
}

fn equal_floats(left: f64, right: f64) -> bool {
    left == right || (left.is_nan() && right.is_nan())
}

fn cmp_floats(left: f64, right: f64) -> Ordering {
    if equal_floats(left, right) {
        Ordering::Equal
    } else if left < right {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}
