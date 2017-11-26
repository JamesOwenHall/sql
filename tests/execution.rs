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
            data_vec![1, 2],
            data_vec![3, 4],
            data_vec![5, 6],
        ],
    );

    let query = sql::parse("select sum(a), sum(b) from bar").unwrap();
    let actual = execute(query, Box::new(input.into_iter())).unwrap();
    let expected = Answer {
        columns: vec![r#"sum("a")"#.to_string(), r#"sum("b")"#.to_string()],
        rows: vec![data_vec![9, 12]],
    };

    assert_eq!(expected, actual);
}

#[test]
fn group_query_execution() {
    let input = make_rows(
        vec!["a", "b"],
        vec![
            data_vec![1, 0],
            data_vec![3, 1],
            data_vec![5, 1],
        ],
    );

    let query = sql::parse("select sum(a), b from bar group by b order by sum(a)").unwrap();
    let actual = execute(query, Box::new(input.into_iter())).unwrap();
    let expected = Answer {
        columns: vec![r#"sum("a")"#.to_string(), r#""b""#.to_string()],
        rows: vec![
            data_vec![1, 0],
            data_vec![8, 1],
        ],
    };

    assert_eq!(expected, actual);
}

#[test]
fn order_by_default_direction() {
    let input = make_rows(
        vec!["a"],
        vec![
            data_vec![3],
            data_vec![2],
            data_vec![1],
        ],
    );

    let query = sql::parse("select a from bar order by a").unwrap();
    let actual = execute(query, Box::new(input.into_iter())).unwrap();
    let expected = Answer {
        columns: vec![r#""a""#.to_string()],
        rows: vec![
            data_vec![1],
            data_vec![2],
            data_vec![3],
        ],
    };

    assert_eq!(expected, actual);
}

#[test]
fn order_by_asc() {
    let input = make_rows(
        vec!["a"],
        vec![
            data_vec![3],
            data_vec![2],
            data_vec![1],
        ],
    );

    let query = sql::parse("select a from bar order by a asc").unwrap();
    let actual = execute(query, Box::new(input.into_iter())).unwrap();
    let expected = Answer {
        columns: vec![r#""a""#.to_string()],
        rows: vec![
            data_vec![1],
            data_vec![2],
            data_vec![3],
        ],
    };

    assert_eq!(expected, actual);
}

#[test]
fn order_by_desc() {
    let input = make_rows(
        vec!["a"],
        vec![
            data_vec![3],
            data_vec![2],
            data_vec![1],
        ],
    );

    let query = sql::parse("select a from bar order by a desc").unwrap();
    let actual = execute(query, Box::new(input.into_iter())).unwrap();
    let expected = Answer {
        columns: vec![r#""a""#.to_string()],
        rows: vec![
            data_vec![3],
            data_vec![2],
            data_vec![1],
        ],
    };

    assert_eq!(expected, actual);
}

#[test]
fn order_by_multiple_fields() {
    let input = make_rows(
        vec!["a", "b"],
        vec![
            data_vec![2, 6],
            data_vec![2, 5],
            data_vec![1, 5],
        ],
    );

    let query = sql::parse("select a, b from bar order by a asc, b desc").unwrap();
    let actual = execute(query, Box::new(input.into_iter())).unwrap();
    let expected = Answer {
        columns: vec![r#""a""#.to_string(), r#""b""#.to_string()],
        rows: vec![
            data_vec![1, 5],
            data_vec![2, 6],
            data_vec![2, 5],
        ],
    };

    assert_eq!(expected, actual);
}