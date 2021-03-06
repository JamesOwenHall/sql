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

    fn execute(&self, source: Source) -> Result<Answer, ExecuteError> {
        let mut source = self.apply_condition(source);
        source = self.compute_aggregates(source)?;
        
        let mut answer = self.apply_select(source)?;
        answer.sort(&self.order_indices);
        Ok(answer)
    }

    fn build_aggregates(&self,
                        source: Source)
                        -> Result<HashMap<Vec<Data>, Vec<Aggregate>>, ExecuteError> {
        let mut groups = HashMap::new();
        for row in source {
            let row = row?;
            let group_aggregates = groups
                .entry(self.build_group(&row))
                .or_insert_with(|| self.make_aggregates());

            for (i, aggregate) in group_aggregates.iter_mut().enumerate() {
                aggregate.apply(self.aggregate_calls[i].argument.eval(&row));
            }
        }

        Ok(groups)
    }

    fn aggregates_to_source(&self, aggregates: HashMap<Vec<Data>, Vec<Aggregate>>) -> Source {
        let mut rows = Vec::with_capacity(aggregates.len());
        for (group, aggregates) in aggregates {
            let mut row = Row::new();
            for (i, aggregate) in aggregates.into_iter().enumerate() {
                let call = self.aggregate_calls[i].clone();
                row.fields.insert(Expr::AggregateCall(call), aggregate.final_value());
            }
            for (index, val) in group.iter().enumerate() {
                row.fields.insert(self.query.group[index].clone(), val.clone());
            }
            rows.push(Ok(row));
        }

        Box::new(rows.into_iter())
    }

    fn apply_condition(&self, source: Source) -> Source {
        let condition = match self.query.condition {
            Some(ref cond) => cond.clone(),
            None => return source,
        };

        Box::new(source.filter(move |row| {
            match row {
                &Err(_) => true,
                &Ok(ref row) => condition.eval(row) == Data::Bool(true),
            }
        }))
    }

    fn compute_aggregates(&self, source: Source) -> Result<Source, ExecuteError> {
        if self.aggregate_calls.is_empty() {
            return Ok(source);
        }

        let aggregates = self.build_aggregates(source)?;
        Ok(self.aggregates_to_source(aggregates))
    }

    fn apply_select(&self, source: Source) -> Result<Answer, ExecuteError> {
        let mut rows = Vec::new();

        for row in source {
            let row = row?;
            let output_row = self.query
                .select
                .iter()
                .map(|field| field.eval(&row))
                .collect();
            rows.push(output_row);
        }

        Ok(Answer {
            columns: self.get_columns(),
            rows: rows,
        })
    }

    fn get_columns(&self) -> Vec<String> {
        self.query.select.iter()
            .map(|field| format!("{}", field))
            .collect()
    }

    fn build_group(&self, row: &Row) -> Vec<Data> {
        self.query.group.iter()
            .map(|field| field.eval(row))
            .collect()
    }

    fn make_aggregates(&self) -> Vec<Aggregate> {
        self.aggregate_calls.iter()
            .map(|call| call.function.aggregate())
            .collect()
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
            columns: vec![String::from("sum(a)")],
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
            columns: vec![String::from("a")],
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
