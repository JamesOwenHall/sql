use data::Data;

pub trait Aggregate {
    fn apply(&mut self, value: Data);
    fn final_value(&self) -> Data;
}

pub struct Sum {
    total: i64,
}

impl Aggregate for Sum {
    fn apply(&mut self, value: Data) {
        if let Data::Int(i) = value {
            self.total += i;
        }
    }

    fn final_value(&self) -> Data {
        Data::Int(self.total)
    }
}
