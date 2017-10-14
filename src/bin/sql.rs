extern crate sql;
extern crate clap;

use clap::{App, Arg};

fn main() {
    let matches = App::new("sql")
        .version("0.1.0")
        .author("James Hall")
        .arg(Arg::with_name("QUERY")
            .help("The query to run")
            .required(true))
        .get_matches();

    let query_str = matches.value_of("QUERY").unwrap();
    let query = sql::parse(query_str).unwrap();
    let source = sql::open_file(&query.from).unwrap();
    let answer = sql::execute(query, source);
    println!("{:?}", answer);
}
