use std::fmt;
use data::Number;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Select,
    From,
    Where,
    Group,
    Order,
    By,
    Asc,
    Desc,
    Identifier(String),
    String(String),
    Number(Number),
    OpenParen,
    CloseParen,
    Comma,
    Eq,
}

impl Token {
    fn format_string(f: &mut fmt::Formatter, input: &str, delimiter: char) -> fmt::Result {
        write!(f, "{}", delimiter)?;
        for c in input.chars() {
            match c {
                '\n' => write!(f, r#"\n"#)?,
                '\\' => write!(f, r#"\"#)?,
                c if c == delimiter => write!(f, r#"\{}"#, delimiter)?,
                c => write!(f, "{}", c)?,
            }
        }

        write!(f, "{}", delimiter)
    }

    fn format_identifier(f: &mut fmt::Formatter, input: &str) -> fmt::Result {
        if Self::is_alphanumeric(input) {
            return write!(f, "{}", input);
        }

        Self::format_string(f, input, '"')
    }

    fn is_alphanumeric(s: &str) -> bool {
        s.chars().all(|c| ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z') || c == '_')
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Token::Select => write!(f, "select"),
            &Token::From => write!(f, "from"),
            &Token::Where => write!(f, "where"),
            &Token::Group => write!(f, "group"),
            &Token::Order => write!(f, "order"),
            &Token::By => write!(f, "by"),
            &Token::Asc => write!(f, "asc"),
            &Token::Desc => write!(f, "desc"),
            &Token::Identifier(ref i) => Self::format_identifier(f, i),
            &Token::String(ref s) => Self::format_string(f, s, '\''),
            &Token::Number(ref n) => write!(f, "{}", n),
            &Token::OpenParen => write!(f, "("),
            &Token::CloseParen => write!(f, ")"),
            &Token::Comma => write!(f, ","),
            &Token::Eq => write!(f, "="),
        }
    }
}
