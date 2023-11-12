use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    PLUS,
    MINUS,
    MULTIPLICATION,
    DIVIDE,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    OPE(Operator),
    IDENTIFIER(String),
    INT(i64),
    FLOAT(f64),
    EQUAL,
    RPAR,
    LPAR,
    Null,
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::PLUS => write!(f, "+"),
            Operator::MINUS => write!(f, "-"),
            Operator::DIVIDE => write!(f, "/"),
            Operator::MULTIPLICATION => write!(f, "*"),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::LPAR => write!(f, "("),
            Token::RPAR => write!(f, ")"),
            Token::EQUAL => write!(f, "="),
            Token::FLOAT(i) => write!(f, "{}", i),
            Token::INT(i) => write!(f, "{}", i),
            Token::IDENTIFIER(s) => write!(f, "{}", s),
            Token::OPE(s) => write!(f, "{}", s),
            Token::Null => write!(f, "Null"),
        }
    }
}

impl Token {
    pub fn priority(&self, other: &Self) -> bool {
        match (&self, other) {
            (Token::OPE(Operator::PLUS), Token::OPE(Operator::MULTIPLICATION)) => true,
            (Token::OPE(Operator::PLUS), Token::OPE(Operator::DIVIDE)) => true,
            (Token::OPE(Operator::MINUS), Token::OPE(Operator::MULTIPLICATION)) => true,
            (Token::OPE(Operator::MINUS), Token::OPE(Operator::DIVIDE)) => true,
            _ => false,
        }
    }
}
