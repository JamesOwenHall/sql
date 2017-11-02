use std::collections::HashMap;
use aggregate::{Aggregate, AggregateCall};
use answer::Answer;
use data::Data;
use expr::Expr;
use query::Query;
use row::Row;
use source::{Source, SourceError};

struct Executor {
    query: Query,
    aggregate_calls: Vec<AggregateCall>,
    order_indices: Vec<usize>,
}

impl Executor {
    fn new(query: Query) -> Self {
        let mut aggregates = Vec::new();
        for expr in query.select.iter() {
            if let Some(call) = expr.get_aggregate_call() {
                aggregates.push(call);
            }
        }

        let mut order_indices = Vec::new();
        for expr in query.order.iter() {
            for (index, select) in query.select.iter().enumerate() {
                if select == expr {
                    order_indices.push(index);
                }
            }
        }

        Executor {
            query: query,
            aggregate_calls: aggregates,
            order_indices: order_indices,
        }
    }

    fn execute(&self, source: Source) -> Result<Answer, SourceError> {
        let mut answer = if self.aggregate_calls.is_empty() {
            self.execute_non_aggregate(source)?
        } else {
            self.execute_aggregate(source)?
        };

        self.sort_answer(&mut answer);
        Ok(answer)
    }

    fn execute_non_aggregate(&self, source: Source) -> Result<Answer, SourceError> {
        let mut rows = Vec::new();
        for input_row in source {
            let input_row = input_row?;
            let mut row = Vec::new();
            for field in self.query.select.iter() {
                row.push(field.eval(&input_row));
            }
            rows.push(row);
        }

        Ok(Answer {
            columns: self.get_columns(),
            rows: rows,
        })
    }

    fn execute_aggregate(&self, source: Source) -> Result<Answer, SourceError> {
        let mut groups = HashMap::new();

        for row in source {
            let row = row?;
            let group = self.get_group(&row);
            let group_aggregates = groups
                .entry(group.clone())
                .or_insert_with(|| self.make_aggregates());

            for (call, aggregate) in group_aggregates.iter_mut() {
                aggregate.apply(call.argument.eval(&row));
            }
        }

        let mut rows = Vec::new();
        for (group, group_aggregates) in groups {
            let mut row = Row::new();
            for (call, aggregate) in group_aggregates {
                row.fields.insert(Expr::AggregateCall(call), aggregate.final_value());
            }
            for (index, val) in group.iter().enumerate() {
                row.fields.insert(self.query.group[index].clone(), val.clone());
            }

            let mut output_row = Vec::new();
            for field in self.query.select.iter() {
                output_row.push(field.eval(&row));
            }
            rows.push(output_row);
        }

        Ok(Answer {
            columns: self.get_columns(),
            rows: rows,
        })
    }

    fn sort_answer(&self, answer: &mut Answer) {
        if !self.order_indices.is_empty() {
            answer.rows.sort_unstable_by_key(|row| {
                let mut sort_keys = Vec::with_capacity(self.order_indices.len());
                for index in self.order_indices.iter() {
                    sort_keys.push(row[*index].clone());
                }
                sort_keys
            });
        }
    }

    fn get_columns(&self) -> Vec<String> {
        let mut columns = Vec::new();
        for field in self.query.select.iter() {
            columns.push(format!("{}", field));
        }
        columns
    }

    fn get_group(&self, row: &Row) -> Vec<Data> {
        let mut group = Vec::new();
        for field in self.query.group.iter() {
            group.push(field.eval(row));
        }
        group
    }

    fn make_aggregates(&self) -> HashMap<AggregateCall, Aggregate> {
        let mut aggregates = HashMap::new();
        for call in self.aggregate_calls.iter() {
            aggregates.insert(call.clone(), call.function.aggregate());
        }
        aggregates
    }
}

pub fn execute(query: Query, source: Source) -> Result<Answer, SourceError> {
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
        let source = make_rows(
            vec!["a"],
            vec![
                row![1],
                row![2],
                row![3],
                row![4],
                row![5],
            ],
        );

        let call = AggregateCall{
            function: AggregateFunction::Sum,
            argument: Box::new(Expr::Column(String::from("a"))),
        };

        let query = Query {
            select: vec![Expr::AggregateCall(call)],
            from: String::new(),
            group: vec![],
            order: vec![],
        };

        let actual = execute(query, Box::new(source.into_iter())).unwrap();
        let expected = Answer {
            columns: vec![String::from(r#"sum("a")"#)],
            rows: vec![row![15]],
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn non_aggregate_query() {
        let source = make_rows(
            vec!["a"],
            vec![
                row![1],
                row![2],
                row![3],
                row![4],
                row![5],
            ],
        );

        let query = Query {
            select: vec![Expr::Column(String::from("a"))],
            from: String::new(),
            group: vec![],
            order: vec![],
        };

        let actual = execute(query, Box::new(source.clone().into_iter())).unwrap();
        let expected = Answer {
            columns: vec![String::from(r#""a""#)],
            rows: vec![
                row![1],
                row![2],
                row![3],
                row![4],
                row![5],
            ],
        };

        assert_eq!(expected, actual);
    }
}
