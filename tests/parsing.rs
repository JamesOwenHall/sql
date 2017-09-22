extern crate sql;

#[test]
fn query_format() {
    let input = "select sum(a), sum(b) from c";
    let query = sql::parse(input).unwrap();
    assert_eq!(input, format!("{}", query));
}
