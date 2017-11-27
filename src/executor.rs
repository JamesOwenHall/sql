use std::collections::HashMap;
use aggregate::{Aggregate, AggregateCall};
use answer::Answer;
use data::Data;
use expr::Expr;
use query::{Query, SortDirection};
use row::Row;
use source::{Source, SourceError};

#[derive(Clone, Debug, PartialEq)]
pub enum ExecuteError {
    SourceError(String),
    InvalidOrderClause(Expr),
}

impl From<SourceError> for ExecuteError {
    fn from(e: SourceError) -> Self {
        ExecuteError::SourceError(e.description)
    }
}

struct Executor {
    query: Query,
    aggregate_calls: Vec<AggregateCall>,
    order_indices: Vec<(usize, SortDirection)>,
}

impl Executor {
    fn new(query: Query) -> Result<Self, ExecuteError> {
        let aggregates = query.select.iter()
            .filter_map(|expr| expr.get_aggregate_call())
            .collect();

        let mut order_indices = Vec::new();
        for sort_field in query.order.iter() {
            let position = query.select.iter().position(|select| select == &sort_field.expr);
            let index = match position {
                Some(i) => i,
                None => return Err(ExecuteError::InvalidOrderClause(sort_field.expr.clone())),
            };
            let direction = sort_field.direction.clone().unwrap_or(SortDirection::Asc);
            order_indices.push((index, direction));
        }

        Ok(Executor {
            query: query,
            aggregate_calls: aggregates,
            order_indices: order_indices,
        })
    }

    fn execute(&self, mut source: Source) -> Result<Answer, ExecuteError> {
        if let Some(condition) = self.query.condition.clone() {
            source = Box::new(source.filter(move |row| {
                match row {
                    &Ok(ref row) => condition.eval(row) == Data::Bool(true),
                    &Err(_) => true,
                }
            }));
        }

        let mut answer = if self.aggregate_calls.is_empty() {
            self.execute_non_aggregate(source)?
        } else {
            self.execute_aggregate(source)?
        };

        answer.sort(&self.order_indices);
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
            let group_aggregates = groups
                .entry(self.get_group(&row))
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

pub fn execute(query: Query, source: Source) -> Result<Answer, ExecuteError> {
    Executor::new(query)?.execute(source)
}

#[cfg(test)]
mod tests {
    use super::*;
    use aggregate::{AggregateCall, AggregateFunction};
    use data::Data;
    use expr::Expr;
    use query::OrderField;
    use row::make_rows;

    #[test]
    fn aggregate_query() {
        let source = make_rows(
            vec!["a"],
            vec![
                data_vec![1],
                data_vec![2],
                data_vec![3],
                data_vec![4],
                data_vec![5],
            ],
        );

        let call = AggregateCall{
            function: AggregateFunction::Sum,
            argument: Box::new(Expr::Column(String::from("a"))),
        };

        let query = Query {
            select: vec![Expr::AggregateCall(call)],
            from: String::new(),
            condition: None,
            group: vec![],
            order: vec![],
        };

        let actual = execute(query, Box::new(source.into_iter())).unwrap();
        let expected = Answer {
            columns: vec![String::from(r#"sum("a")"#)],
            rows: vec![data_vec![15]],
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn non_aggregate_query() {
        let source = make_rows(
            vec!["a"],
            vec![
                data_vec![1],
                data_vec![2],
                data_vec![3],
                data_vec![4],
                data_vec![5],
            ],
        );

        let query = Query {
            select: vec![Expr::Column(String::from("a"))],
            from: String::new(),
            condition: None,
            group: vec![],
            order: vec![],
        };

        let actual = execute(query, Box::new(source.clone().into_iter())).unwrap();
        let expected = Answer {
            columns: vec![String::from(r#""a""#)],
            rows: vec![
                data_vec![1],
                data_vec![2],
                data_vec![3],
                data_vec![4],
                data_vec![5],
            ],
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn invalid_order_clause() {
        let source = make_rows(
            vec!["a"],
            vec![],
        );

        let query = Query {
            select: vec![],
            from: String::new(),
            condition: None,
            group: vec![],
            order: vec![OrderField {
                expr: Expr::Column(String::from("a")),
                direction: None,
            }],
        };
        let actual = execute(query, Box::new(source.clone().into_iter()));
        let expected = Err(ExecuteError::InvalidOrderClause(Expr::Column(String::from("a"))));
        assert_eq!(expected, actual);
    }
}
