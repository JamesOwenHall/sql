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
        let select = self.parse_expr()?;
        self.expect(Token::From)?;
        Ok(Query{
            select: vec![select],
            from: String::new(),
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
    fn unknown_function() {
        let input = "select blah(value) from foo";
        let actual = parse(input);
        let expected = Err(ParseError::UnknownFunction(String::from("blah")));
        assert_eq!(expected, actual);
    }
}
