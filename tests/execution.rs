extern crate sql;

use sql::Answer;
use sql::Data;
use sql::execute;
use sql::row::make_rows;

#[test]
fn query_execution() {
    let input = make_rows(vec!["a", "b"], vec![
        vec![Data::Int(1), Data::Int(2)],
        vec![Data::Int(3), Data::Int(4)],
        vec![Data::Int(5), Data::Int(6)],
    ]);

    let query = sql::parse("select sum(a), sum(b) from bar").unwrap();
    let actual = execute(query, Box::new(input.into_iter()));
    let expected = Answer {
        columns: vec!["sum(a)".to_string(), "sum(b)".to_string()],
        rows: vec![vec![Data::Int(9), Data::Int(12)]],
    };

    assert_eq!(expected, actual);
}
