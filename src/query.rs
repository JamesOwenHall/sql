use std::fmt;
use expr::Expr;

#[derive(Clone, Debug, PartialEq)]
pub struct Query {
    pub select: Vec<Expr>,
    pub from: String,
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let select: Vec<String> = self.select.iter()
            .map(|expr| format!("{}", expr))
            .collect();
        write!(f, "select {} from {}", select.join(", "), self.from)
    }
}
