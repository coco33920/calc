use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Operator {
    PLUS,
    MINUS,
    MULTIPLICATION,
    DIVIDE,
}

#[derive(Debug)]
pub enum Token {
    OPE(Operator),
    IDENTIFIER(String),
    INT(i64),
    FLOAT(f64),
    EQUAL,
    RPAR,
    LPAR,
    QUOTE,
}

impl PartialEq<Self> for Operator {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Operator::PLUS, Operator::PLUS) => true,
            (Operator::MINUS, Operator::MINUS) => true,
            (Operator::MULTIPLICATION, Operator::MULTIPLICATION) => true,
            (Operator::DIVIDE, Operator::DIVIDE) => true,
            _ => false
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::PLUS => write!(f, "+"),
            Operator::MINUS => write!(f, "-"),
            Operator::DIVIDE => write!(f, "/"),
            Operator::MULTIPLICATION => write!(f, "*")
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::LPAR => write!(f, "("),
            Token::RPAR => write!(f, ")"),
            Token::QUOTE => write!(f, "\""),
            Token::EQUAL => write!(f, "="),
            Token::FLOAT(i) => write!(f, "{}", i),
            Token::INT(i) => write!(f, "{}", i),
            Token::IDENTIFIER(s) => write!(f, "{}", s),
            Token::OPE(s) => write!(f, "{}", s)
        }
    }
}

impl PartialEq<Self> for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::LPAR, Token::LPAR) => true,
            (Token::RPAR, Token::RPAR) => true,
            (Token::QUOTE, Token::QUOTE) => true,
            (Token::EQUAL, Token::EQUAL) => true,
            (Token::FLOAT(i), Token::FLOAT(i2)) => i == i2,
            (Token::INT(i), Token::INT(i2)) => i == i2,
            (Token::IDENTIFIER(s), Token::IDENTIFIER(s2)) => s == s2,
            (Token::OPE(o), Token::OPE(p)) => o == p,
            _ => false
        }
    }
}
