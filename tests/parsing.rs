extern crate sql;

#[test]
fn query_format() {
    let inputs = vec![
        "select a, b from c",
        "select sum(a), sum(b) from c",
        "select sum(a), b from c group by b",
    ];

    for input in inputs {
        let query = sql::parse(input).unwrap();
        assert_eq!(input, format!("{}", query));
    }
}
