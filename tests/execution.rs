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

    let query = sql::parse("select sum(a) from bar").unwrap();
    let actual = execute(query, Box::new(input.into_iter()));
    let expected = Answer {
        columns: vec![String::from("sum(a)")],
        rows: vec![vec![Data::Int(9)]],
    };

    assert_eq!(expected, actual);
}
