extern crate sql;

use sql::{Data, Number};
use sql::parser::Parser;
use sql::row::Row;

#[test]
fn numbers() {
    run_expr(Data::Number(Number::Int(0)), "0", None);
    run_expr(Data::Number(Number::Float(0.0)), "0.0", None);
    run_expr(Data::Number(Number::Int(1)), "1", None);
    run_expr(Data::Number(Number::Float(3.14)), "3.14", None);
}

#[test]
fn binary_exprs() {
    run_expr(Data::Bool(true), "0 = 0", None);
    run_expr(Data::Bool(false), "1 = 0", None);
    run_expr(Data::Bool(true), "a = a", None);
}

fn run_expr(expected: Data, expr: &str, row: Option<Row>) {
    let row = if let Some(r) = row {
        r
    } else {
        Row::new()
    };

    let mut parser = Parser::new(expr);
    let expr = parser.parse_expr().unwrap();
    println!("expr: {}", expr);
    let actual = expr.eval(&row);
    assert_eq!(expected, actual);
}
