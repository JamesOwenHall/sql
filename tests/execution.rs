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
    let actual = execute(query, Box::new(input.into_iter())).unwrap();
    let expected = Answer {
        columns: vec![r#"sum("a")"#.to_string(), r#"sum("b")"#.to_string()],
        rows: vec![vec![Data::Int(9), Data::Int(12)]],
    };

    assert_eq!(expected, actual);
}

#[test]
fn group_query_execution() {
    let input = make_rows(vec!["a", "b"], vec![
        vec![Data::Int(1), Data::Int(0)],
        vec![Data::Int(3), Data::Int(1)],
        vec![Data::Int(5), Data::Int(1)],
    ]);

    let query = sql::parse("select sum(a), b from bar group by b order by sum(a)").unwrap();
    let actual = execute(query, Box::new(input.into_iter())).unwrap();
    let expected = Answer {
        columns: vec![r#"sum("a")"#.to_string(), r#""b""#.to_string()],
        rows: vec![
            vec![Data::Int(1), Data::Int(0)],
            vec![Data::Int(8), Data::Int(1)],
        ],
    };

    assert_eq!(expected, actual);
}

#[test]
fn order_by_execution() {
    let input = make_rows(vec!["a"], vec![
        vec![Data::Int(3)],
        vec![Data::Int(2)],
        vec![Data::Int(1)],
    ]);

    let query = sql::parse("select a from bar order by a").unwrap();
    let actual = execute(query, Box::new(input.into_iter())).unwrap();
    let expected = Answer {
        columns: vec![r#""a""#.to_string()],
        rows: vec![
            vec![Data::Int(1)],
            vec![Data::Int(2)],
            vec![Data::Int(3)],
        ],
    };

    assert_eq!(expected, actual);
}
