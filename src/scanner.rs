use std::iter::Peekable;
use std::str::{Chars, FromStr};
use data::Number;
use parser::ParseError;
use token::Token;

#[derive(Clone, Debug, PartialEq)]
pub enum ScanError {
    UnexpectedEOF,
    UnknownToken(char),
}

impl Into<ParseError> for ScanError {
    fn into(self) -> ParseError {
        match self {
            ScanError::UnexpectedEOF => ParseError::UnexpectedEOF,
            ScanError::UnknownToken(c) => ParseError::UnknownToken(c),
        }
    }
}

type Result<A> = ::std::result::Result<A, ScanError>;

#[derive(Clone, Debug)]
pub struct Scanner<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_spaces();

        let next = match self.input.peek() {
            None => return None,
            Some(c) => *c,
        };

        Some(match next {
            '(' => {
                self.input.next();
                Ok(Token::OpenParen)
            },
            ')' => {
                self.input.next();
                Ok(Token::CloseParen)
            },
            ',' => {
                self.input.next();
                Ok(Token::Comma)
            },
            '=' => {
                self.input.next();
                Ok(Token::Eq)
            },
            '\'' => self.read_string(),
            '"' => self.read_quoted_identifier(),
            c if Self::is_letter(c) => Ok(self.read_identifier()),
            c if Self::is_digit(c) => self.read_number(),
            c => Err(ScanError::UnknownToken(c)),
        })
    }
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        Scanner{input: input.chars().peekable()}
    }

    fn skip_spaces(&mut self) {
        loop {
            match self.input.peek() {
                None => return,
                Some(c) if !Self::is_space(*c) => return,
                _ => {},
            };
            self.input.next();
        }
    }

    fn read_identifier(&mut self) -> Token {
        let mut buf = String::new();
        loop {
            match self.input.peek().cloned() {
                Some(c) if Self::is_letter(c) || Self::is_digit(c) => {
                    self.input.next();
                    buf.push(c)
                },
                _ => break,
            };
        }

        match buf.to_lowercase().as_ref() {
            "select" => Token::Select,
            "from" => Token::From,
            "where" => Token::Where,
            "group" => Token::Group,
            "order" => Token::Order,
            "by" => Token::By,
            "asc" => Token::Asc,
            "desc" => Token::Desc,
            _ => Token::Identifier(buf),
        }
    }

    fn read_string(&mut self) -> Result<Token> {
        self.read_quoted_token('\'').map(|s| Token::String(s))
    }

    fn read_quoted_identifier(&mut self) -> Result<Token> {
        self.read_quoted_token('"').map(|s| Token::Identifier(s))
    }

    fn read_quoted_token(&mut self, delimiter: char) -> Result<String> {
        assert_eq!(Some(delimiter), self.input.next());

        let mut string = String::new();
        loop {
            match self.input.next() {
                None => return Err(ScanError::UnexpectedEOF),
                Some(c) if c == delimiter => return Ok(string),
                Some('\\') => {
                    match self.input.next() {
                        Some('n') => string.push('\n'),
                        Some(c) => string.push(c),
                        None => return Err(ScanError::UnexpectedEOF),
                    }
                },
                Some(c) => string.push(c),
            }
        }
    }

    fn read_number(&mut self) -> Result<Token> {
        let mut buf = String::new();
        let mut saw_decimal_point = false;

        loop {
            match self.input.peek() {
                Some(&'.') if !saw_decimal_point => {
                    saw_decimal_point = true;
                    buf.push('.');
                },
                Some(&c) if Self::is_digit(c) => buf.push(c),
                Some(_) => break,
                None => break,
            }

            self.input.next();
        }

        let num = if saw_decimal_point {
            Number::Float(f64::from_str(&buf).unwrap())
        } else {
            Number::Int(i64::from_str(&buf).unwrap())
        };

        Ok(Token::Number(num))
    }

    fn is_space(c: char) -> bool {
        c == ' ' || c == '\t' || c == '\r' || c == '\n'
    }

    fn is_letter(c: char) -> bool {
        ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z') || c == '_'
    }

    fn is_digit(c: char) -> bool {
        '0' <= c && c <= '9'
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use data::Number;

    #[test]
    fn symbols() {
        let mut scanner = Scanner::new("(,)=");
        assert_eq!(scanner.next(), Some(Ok(Token::OpenParen)));
        assert_eq!(scanner.next(), Some(Ok(Token::Comma)));
        assert_eq!(scanner.next(), Some(Ok(Token::CloseParen)));
        assert_eq!(scanner.next(), Some(Ok(Token::Eq)));
        assert_eq!(scanner.next(), None);
    }

    #[test]
    fn identifiers() {
        let mut scanner = Scanner::new(r#"select FrOm foo where group order by asc desc "a field""#);
        assert_eq!(scanner.next(), Some(Ok(Token::Select)));
        assert_eq!(scanner.next(), Some(Ok(Token::From)));
        assert_eq!(scanner.next(), Some(Ok(Token::Identifier(("foo".to_string())))));
        assert_eq!(scanner.next(), Some(Ok(Token::Where)));
        assert_eq!(scanner.next(), Some(Ok(Token::Group)));
        assert_eq!(scanner.next(), Some(Ok(Token::Order)));
        assert_eq!(scanner.next(), Some(Ok(Token::By)));
        assert_eq!(scanner.next(), Some(Ok(Token::Asc)));
        assert_eq!(scanner.next(), Some(Ok(Token::Desc)));
        assert_eq!(scanner.next(), Some(Ok(Token::Identifier("a field".to_string()))));
        assert_eq!(scanner.next(), None);
    }

    #[test]
    fn strings() {
        let mut scanner = Scanner::new(r#"'' 'foo' '\'' '\n' '\\'"#);
        assert_eq!(scanner.next(), Some(Ok(Token::String("".to_string()))));
        assert_eq!(scanner.next(), Some(Ok(Token::String("foo".to_string()))));
        assert_eq!(scanner.next(), Some(Ok(Token::String("'".to_string()))));
        assert_eq!(scanner.next(), Some(Ok(Token::String("\n".to_string()))));
        assert_eq!(scanner.next(), Some(Ok(Token::String("\\".to_string()))));
    }

    #[test]
    fn numbers() {
        let mut scanner = Scanner::new("1 123 123.25 3.0");
        assert_eq!(scanner.next(), Some(Ok(Token::Number(Number::Int(1)))));
        assert_eq!(scanner.next(), Some(Ok(Token::Number(Number::Int(123)))));
        assert_eq!(scanner.next(), Some(Ok(Token::Number(Number::Float(123.25)))));
        assert_eq!(scanner.next(), Some(Ok(Token::Number(Number::Float(3.0)))));
        assert_eq!(scanner.next(), None);
    }

    #[test]
    fn unknown_token() {
        let mut scanner = Scanner::new("^");
        assert_eq!(scanner.next(), Some(Err(ScanError::UnknownToken('^'))));
    }
}
