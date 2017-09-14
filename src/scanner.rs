use std::iter::Peekable;
use std::str::Chars;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Select,
    From,
    Identifier(String),
    OpenParen,
    CloseParen,
    Comma,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ScanError {
    UnexpectedEOF,
    UnknownToken(char)
}

type Result = ::std::result::Result<Token, ScanError>;

pub struct Scanner<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Result;

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
            c if Self::is_letter(c) => Ok(self.read_identifier()),
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
            _ => Token::Identifier(buf),
        }
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

    #[test]
    fn symbols() {
        let mut scanner = Scanner::new("(,)");
        assert_eq!(scanner.next(), Some(Ok(Token::OpenParen)));
        assert_eq!(scanner.next(), Some(Ok(Token::Comma)));
        assert_eq!(scanner.next(), Some(Ok(Token::CloseParen)));
        assert_eq!(scanner.next(), None);
    }

    #[test]
    fn identifiers() {
        let mut scanner = Scanner::new("select FrOm foo");
        assert_eq!(scanner.next(), Some(Ok(Token::Select)));
        assert_eq!(scanner.next(), Some(Ok(Token::From)));
        assert_eq!(scanner.next(), Some(Ok(Token::Identifier(String::from("foo")))));
        assert_eq!(scanner.next(), None);
    }

    #[test]
    fn unknown_token() {
        let mut scanner = Scanner::new("^");
        assert_eq!(scanner.next(), Some(Err(ScanError::UnknownToken('^'))));
    }
}
