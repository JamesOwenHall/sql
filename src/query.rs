use std::fmt;
use expr::Expr;

#[derive(Clone, Debug, PartialEq)]
pub struct Query {
    pub select: Vec<Expr>,
    pub from: String,
    pub group: Vec<Expr>,
    pub order: Vec<Expr>,
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let select: Vec<String> = self.select.iter()
            .map(|expr| format!("{}", expr))
            .collect();
        write!(f, r#"select {} from "{}""#, select.join(", "), self.from)?;

        if !self.group.is_empty() {
            let group: Vec<String> = self.group.iter()
                .map(|expr| format!("{}", expr))
                .collect();
            write!(f, " group by {}", group.join(", "))?;
        }

        if !self.order.is_empty() {
            let order: Vec<String> = self.order.iter()
                .map(|expr| format!("{}", expr))
                .collect();
            write!(f, " order by {}", order.join(", "))?;
        }

        Ok(())
    }
}
