use std::iter::Peekable;
use aggregate::{AggregateCall, AggregateFunction};
use expr::Expr;
use query::Query;
use scanner::{Scanner, Token};

#[derive(Clone, Debug)]
pub struct Parser<'a> {
    scanner: Peekable<Scanner<'a>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ParseError {
    UnexpectedEOF,
    UnknownToken(char),
    UnexpectedToken(Token),
    UnknownFunction(String),
}

type Result<A> = ::std::result::Result<A, ParseError>;

pub fn parse(input: &str) -> Result<Query> {
    Parser::new(input).parse()
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser{scanner: Scanner::new(input).peekable()}
    }

    pub fn parse(&mut self) -> Result<Query> {
        self.expect(Token::Select)?;
        let select = self.parse_select()?;
        
        self.expect(Token::From)?;
        let from = match self.scanner.next() {
            Some(Ok(Token::Identifier(i))) => i,
            Some(Ok(t)) => return Err(ParseError::UnexpectedToken(t)),
            Some(Err(e)) => return Err(e.into()),
            None => return Err(ParseError::UnexpectedEOF),
        };

        let group = match self.scanner.peek().cloned() {
            Some(Ok(Token::Group)) => self.parse_group_by()?,
            _ => vec![],
        };

        let order = match self.scanner.peek().cloned() {
            Some(Ok(Token::Order)) => self.parse_order_by()?,
            _ => vec![],
        };

        Ok(Query {
            select: select,
            from: from,
            group: group,
            order: order,
        })
    }

    pub fn parse_expr(&mut self) -> Result<Expr> {
        match self.scanner.next() {
            None => Err(ParseError::UnexpectedEOF),
            Some(Ok(Token::Identifier(i))) => self.parse_identifier(i),
            Some(Err(e)) => Err(e.into()),
            Some(Ok(t)) => Err(ParseError::UnexpectedToken(t)),
        }
    }

    fn parse_select(&mut self) -> Result<Vec<Expr>> {
        let mut exprs = Vec::new();
        loop {
            exprs.push(self.parse_expr()?);
            match self.scanner.peek().cloned() {
                Some(Ok(Token::Comma)) => self.scanner.next(),
                _ => return Ok(exprs),
            };
        }
    }

    fn parse_identifier(&mut self, identifier: String) -> Result<Expr> {
        if let Some(&Ok(Token::OpenParen)) = self.scanner.peek() {
            self.scanner.next();
            let argument = self.parse_expr()?;
            self.expect(Token::CloseParen)?;

            let aggregate_function = match AggregateFunction::from_name(&identifier) {
                Some(func) => func,
                None => return Err(ParseError::UnknownFunction(identifier)),
            };

            Ok(Expr::AggregateCall(AggregateCall{
                function: aggregate_function,
                argument: Box::new(argument),
            }))
        } else {
            Ok(Expr::Column(identifier))
        }
    }

    fn parse_group_by(&mut self) -> Result<Vec<Expr>> {
        self.expect(Token::Group)?;
        self.expect(Token::By)?;
        self.parse_comma_separated_exprs()
    }

    fn parse_order_by(&mut self) -> Result<Vec<Expr>> {
        self.expect(Token::Order)?;
        self.expect(Token::By)?;
        self.parse_comma_separated_exprs()
    }

    fn parse_comma_separated_exprs(&mut self) -> Result<Vec<Expr>> {
        let mut exprs = Vec::new();
        loop {
            exprs.push(self.parse_expr()?);
            match self.scanner.peek().cloned() {
                Some(Ok(Token::Comma)) => self.scanner.next(),
                _ => return Ok(exprs),
            };
        }
    }

    fn expect(&mut self, t: Token) -> Result<()> {
        match self.scanner.next() {
            Some(Ok(ref token)) if *token == t => Ok(()),
            Some(Ok(token)) => Err(ParseError::UnexpectedToken(token)),
            Some(Err(e)) => Err(e.into()),
            None => Err(ParseError::UnexpectedEOF),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_aggregate_query() {
        let input = "select sum(value) from foo";
        parse(input).unwrap();
    }

    #[test]
    fn parse_group_query() {
        let input = "select sum(a), b from foo group by b";
        parse(input).unwrap();
    }

    #[test]
    fn parse_order_query() {
        let input = "select sum(a), b from foo order by b";
        parse(input).unwrap();
    }

    #[test]
    fn unknown_function() {
        let input = "select blah(value) from foo";
        let actual = parse(input);
        let expected = Err(ParseError::UnknownFunction(String::from("blah")));
        assert_eq!(expected, actual);
    }
}
