use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::mem::transmute;

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub enum Data {
    Null,
    Bool(bool),
    Int(i64),
    Float(Float),
    String(String),
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (&Data::Null, &Data::Null) => Ordering::Equal,
            (&Data::Null, _) => Ordering::Less,
            (&Data::Bool(_), &Data::Null) => Ordering::Greater,
            (&Data::Bool(ref b1), &Data::Bool(ref b2)) => b1.cmp(b2),
            (&Data::Bool(_), _) => Ordering::Less,
            (&Data::Int(_), &Data::Null) => Ordering::Greater,
            (&Data::Int(_), &Data::Bool(_)) => Ordering::Greater,
            (&Data::Int(ref i1), &Data::Int(ref i2)) => i1.cmp(i2),
            (&Data::Int(ref i), &Data::Float(ref f)) => Float{val: (*i as f64)}.cmp(f),
            (&Data::Int(_), _) => Ordering::Less,
            (&Data::Float(_), &Data::Null) => Ordering::Greater,
            (&Data::Float(_), &Data::Bool(_)) => Ordering::Greater,
            (&Data::Float(ref f), &Data::Int(ref i)) => f.cmp(&Float{val: (*i as f64)}),
            (&Data::Float(ref f1), &Data::Float(ref f2)) => f1.cmp(f2),
            (&Data::Float(_), _) => Ordering::Less,
            (&Data::String(ref s1), &Data::String(ref s2)) => s1.cmp(s2),
            (&Data::String(_), _) => Ordering::Greater,
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Float {
    pub val: f64
}

impl Hash for Float {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let t: u64 = unsafe{ transmute(self.val) };
        t.hash(state)
    }
}

impl Eq for Float {}
impl Ord for Float {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.val == other.val || (self.val.is_nan() && other.val.is_nan()) {
            Ordering::Equal
        } else if self.val < other.val {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}
