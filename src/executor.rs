use std::collections::HashMap;
use aggregate::AggregateCall;
use answer::Answer;
use expr::Expr;
use query::Query;
use row::Row;

struct Executor {
    query: Query,
    aggregate_calls: Vec<AggregateCall>,
}

impl Executor {
    fn new(query: Query) -> Self {
        let mut aggregates = Vec::new();

        for expr in query.select.iter() {
            if let Some(call) = expr.get_aggregate_call() {
                aggregates.push(call);
            }
        }

        Executor {
            query: query,
            aggregate_calls: aggregates,
        }
    }

    fn execute(&self, source: Box<Iterator<Item=Row>>) -> Answer {
        if self.aggregate_calls.is_empty() {
            self.execute_non_aggregate(source)
        } else {
            self.execute_aggregate(source)
        }
    }

    fn execute_non_aggregate(&self, source: Box<Iterator<Item=Row>>) -> Answer {
        let mut rows = Vec::new();
        for input_row in source {
            let mut row = Vec::new();
            for field in self.query.select.iter() {
                row.push(field.eval(&input_row));
            }
            rows.push(row);
        }

        Answer {
            columns: self.get_columns(),
            rows: rows,
        }
    }

    fn execute_aggregate(&self, source: Box<Iterator<Item=Row>>) -> Answer {
        let mut aggregates = HashMap::new();
        for call in self.aggregate_calls.iter() {
            aggregates.insert(call.clone(), call.function.aggregate());
        }

        for row in source {
            for (call, aggregate) in aggregates.iter_mut() {
                aggregate.apply(call.argument.eval(&row));
            }
        }

        let mut aggregate_row = Row::new();
        for (call, aggregate) in aggregates {
            aggregate_row.fields.insert(Expr::AggregateCall(call), aggregate.final_value());
        }

        let mut output_row = Vec::new();
        for field in self.query.select.iter() {
            output_row.push(field.eval(&aggregate_row));
        }

        Answer {
            columns: self.get_columns(),
            rows: vec![output_row],
        }
    }

    fn get_columns(&self) -> Vec<String> {
        let mut columns = Vec::new();
        for field in self.query.select.iter() {
            columns.push(format!("{}", field));
        }
        columns
    }
}

pub fn execute(query: Query, source: Box<Iterator<Item=Row>>) -> Answer {
    Executor::new(query).execute(source)
}

#[cfg(test)]
mod tests {
    use super::*;
    use aggregate::{AggregateCall, AggregateFunction};
    use data::Data;
    use expr::Expr;
    use row::make_rows;

    #[test]
    fn aggregate_query() {
        let source = make_rows(vec!["a"], vec![
            vec![Data::Int(1)],
            vec![Data::Int(2)],
            vec![Data::Int(3)],
            vec![Data::Int(4)],
            vec![Data::Int(5)],
        ]);

        let call = AggregateCall{
            function: AggregateFunction::Sum,
            argument: Box::new(Expr::Column(String::from("a"))),
        };

        let query = Query {
            select: vec![Expr::AggregateCall(call)],
            from: String::new(),
        };

        let actual = execute(query, Box::new(source.into_iter()));
        let expected = Answer {
            columns: vec![String::from("sum(a)")],
            rows: vec![vec![Data::Int(15)]],
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn non_aggregate_query() {
        let source = make_rows(vec!["a"], vec![
            vec![Data::Int(1)],
            vec![Data::Int(2)],
            vec![Data::Int(3)],
            vec![Data::Int(4)],
            vec![Data::Int(5)],
        ]);

        let query = Query {
            select: vec![Expr::Column(String::from("a"))],
            from: String::new(),
        };

        let actual = execute(query, Box::new(source.clone().into_iter()));
        let expected = Answer {
            columns: vec![String::from("a")],
            rows: vec![
                vec![Data::Int(1)],
                vec![Data::Int(2)],
                vec![Data::Int(3)],
                vec![Data::Int(4)],
                vec![Data::Int(5)],
            ],
        };

        assert_eq!(expected, actual);
    }
}
