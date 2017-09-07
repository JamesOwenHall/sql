use expr::Expr;

pub struct Query {
    select: Vec<Expr>,
    from: String,
}
