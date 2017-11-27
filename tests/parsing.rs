extern crate sql;

#[test]
fn query_format() {
    let inputs = vec![
        r#"select "a", "b" from "c""#,
        r#"select sum("a"), sum("b") from "c""#,
        r#"select sum("a"), "b" from "c" group by "b""#,
        r#"select "a", "b" from "c" order by "b""#,
        r#"select "a", "b" from "c" group by "a" order by "b""#,
        r#"select "a", "b" from "c" where "a""#,
    ];

    for input in inputs {
        let query = sql::parse(input).unwrap();
        assert_eq!(input, format!("{}", query));
    }
}
