use expr::Expr;

#[derive(Clone, Debug, PartialEq)]
pub struct Query {
    pub select: Vec<Expr>,
    pub from: String,
}
