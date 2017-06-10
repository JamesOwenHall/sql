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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;
    use aggregate::Aggregate;
    use expr::Expr;
    use row::Row;
    use table::VecTable;

    #[test]
    fn execute() {
        let mut table = VecTable::new();
        table.rows.push({
            let mut fields = HashMap::new();
            fields.insert(String::from("a"), Data::Int(0));
            fields.insert(String::from("b"), Data::Int(1));
            fields.insert(String::from("c"), Data::Int(2));
            Row::with_fields(fields)
        });
        table.rows.push({
            let mut fields = HashMap::new();
            fields.insert(String::from("a"), Data::Int(3));
            fields.insert(String::from("b"), Data::Int(4));
            fields.insert(String::from("c"), Data::Int(5));
            Row::with_fields(fields)
        });
        table.rows.push({
            let mut fields = HashMap::new();
            fields.insert(String::from("a"), Data::Int(6));
            fields.insert(String::from("b"), Data::Int(7));
            fields.insert(String::from("c"), Data::Int(8));
            Row::with_fields(fields)
        });

        let mut query = Query {
            calls: vec![
                AggregateCall{
                    aggregate: Aggregate::Avg(0, 0),
                    expr: Expr::Column(String::from("a")),
                },
                AggregateCall{
                    aggregate: Aggregate::Sum(0),
                    expr: Expr::Column(String::from("c")),
                },
            ],
            table: Box::new(table),
        };

        let exp = vec![Data::Int(3), Data::Int(15)];
        assert_eq!(exp, query.execute());
    }
}
