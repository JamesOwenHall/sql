#[macro_use]
extern crate sql;

use sql::Answer;
use sql::Data;
use sql::execute;
use sql::row::make_rows;

#[test]
fn query_execution() {
    let input = make_rows(
        vec!["a", "b"],
        vec![
            row![1, 2],
            row![3, 4],
            row![5, 6],
        ],
    );

    let query = sql::parse("select sum(a), sum(b) from bar").unwrap();
    let actual = execute(query, Box::new(input.into_iter())).unwrap();
    let expected = Answer {
        columns: vec![r#"sum("a")"#.to_string(), r#"sum("b")"#.to_string()],
        rows: vec![row![9, 12]],
    };

    assert_eq!(expected, actual);
}

#[test]
fn group_query_execution() {
    let input = make_rows(
        vec!["a", "b"],
        vec![
            row![1, 0],
            row![3, 1],
            row![5, 1],
        ],
    );

    let query = sql::parse("select sum(a), b from bar group by b order by sum(a)").unwrap();
    let actual = execute(query, Box::new(input.into_iter())).unwrap();
    let expected = Answer {
        columns: vec![r#"sum("a")"#.to_string(), r#""b""#.to_string()],
        rows: vec![
            row![1, 0],
            row![8, 1],
        ],
    };

    assert_eq!(expected, actual);
}

#[test]
fn order_by_execution() {
    let input = make_rows(
        vec!["a"],
        vec![
            row![3],
            row![2],
            row![1],
        ],
    );

    let query = sql::parse("select a from bar order by a").unwrap();
    let actual = execute(query, Box::new(input.into_iter())).unwrap();
    let expected = Answer {
        columns: vec![r#""a""#.to_string()],
        rows: vec![
            row![1],
            row![2],
            row![3],
        ],
    };

    assert_eq!(expected, actual);
}
