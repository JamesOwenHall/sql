use expr::Expr;

pub struct Query {
    pub select: Vec<Expr>,
    pub from: String,
}
