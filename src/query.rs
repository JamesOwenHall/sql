use std::fmt;
use expr::Expr;
use token::Token;

#[derive(Clone, Debug, PartialEq)]
pub struct Query {
    pub select: Vec<Expr>,
    pub from: String,
    pub condition: Option<Expr>,
    pub group: Vec<Expr>,
    pub order: Vec<OrderField>,
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let select: Vec<String> = self.select.iter()
            .map(|expr| format!("{}", expr))
            .collect();
        write!(f, "{} {}", Token::Select, select.join(", "))?;

        write!(f, " {} {}", Token::From, Token::Identifier(self.from.clone()))?;

        if let Some(ref condition) = self.condition {
            write!(f, " {} {}", Token::Where, condition)?;
        }

        if !self.group.is_empty() {
            let group: Vec<String> = self.group.iter()
                .map(|expr| format!("{}", expr))
                .collect();
            write!(f, " {} {} {}", Token::Group, Token::By, group.join(", "))?;
        }

        if !self.order.is_empty() {
            let order: Vec<String> = self.order.iter()
                .map(|expr| format!("{}", expr))
                .collect();
            write!(f, " {} {} {}", Token::Order, Token::By, order.join(", "))?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OrderField {
    pub expr: Expr,
    pub direction: Option<SortDirection>,
}

impl fmt::Display for OrderField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.expr)?;

        if let Some(ref direction) = self.direction {
            write!(f, " {}", direction)?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum SortDirection {
    Asc,
    Desc,
}

impl fmt::Display for SortDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &SortDirection::Asc => write!(f, "asc"),
            &SortDirection::Desc => write!(f, "desc"),
        }
    }
}
