use answer::Answer;
use query::Query;
use row::Row;

pub fn execute(query: Query, source: Box<Iterator<Item=Row>>) -> Answer {
    let mut aggregates = Vec::new();
    let mut non_aggregates = Vec::new();

    for expr in query.select.iter() {
        if let Some(call) = expr.get_aggregate_call() {
            let aggregate = call.function.aggregate();
            aggregates.push((aggregate, call));
        } else {
            non_aggregates.push(expr.clone());
        }
    }

    let output_rows = if aggregates.len() > 0 {
        for row in source {
            for tuple in aggregates.iter_mut() {
                let &mut(ref mut aggregate, ref call) = tuple;
                aggregate.apply(call.argument.eval(&row));
            }
        }

        let mut aggregate_row = Row::new();
        for tuple in aggregates.iter() {
            let &(ref aggregate, ref call) = tuple;
            aggregate_row.aggregates.insert(call.clone(), aggregate.final_value());
        }

        vec![aggregate_row]
    } else {
        let mut rows = Vec::new();
        for input_row in source {
            let mut row = Row::new();
            for field in non_aggregates.iter() {
                let name = format!("{}", field);
                row.fields.insert(name, field.eval(&input_row));
            }
            rows.push(row);
        }

        rows
    };

    let mut columns = Vec::new();
    for field in query.select.iter() {
        columns.push(format!("{}", field));
    }

    let mut rows = Vec::new();
    for output_row in output_rows {
        let mut row = Vec::new();
        for field in query.select.iter() {
            row.push(field.eval(&output_row));
        }
        rows.push(row);
    }

    Answer{columns: columns, rows: rows}
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
