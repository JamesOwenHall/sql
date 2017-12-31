extern crate sql;

#[test]
fn query_format() {
    let inputs = vec![
        "select a, b from c",
        "select sum(a), sum(b) from c",
        "select sum(a), b from c group by b",
        "select a, b from c order by b",
        "select a, b from c group by a order by b",
        "select a, b from c where a",
        r#"select a, b from "fixtures/accounts.json""#,
    ];

    for input in inputs {
        let query = sql::parse(input).unwrap();
        assert_eq!(input, format!("{}", query));
    }
}
