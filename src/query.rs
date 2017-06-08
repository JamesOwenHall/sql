use aggregate::AggregateCall;
use data::Data;
use table::Table;

#[derive(Debug)]
pub struct Query {
    calls: Vec<AggregateCall>,
    table: Box<Table>,
}

impl Query {
    pub fn execute(&mut self) -> Vec<Data> {
        self.apply_rows();
        self.get_output_row()
    }

    fn apply_rows(&mut self) {
        let mut calls = Vec::new();
        calls = ::std::mem::replace(&mut self.calls, calls);

        self.table.for_each(&mut |row| {
            for call in calls.iter_mut() {
                call.apply(row);
            }
        });

        self.calls = calls;
    }

    fn get_output_row(&self) -> Vec<Data> {
        let mut out_row = Vec::new();
        for call in self.calls.iter() {
            out_row.push(call.aggregate.value());
        }

        out_row
    }
}
