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

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    OPE,
    IDENTIFIER,
    INT,
    FLOAT,
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

impl Operator {
    fn priority(&self) -> i64 {
        match &self {
            Operator::PLUS => 3,
            Operator::MINUS => 4,
            Operator::MULTIPLICATION => 5,
            Operator::DIVIDE => 6,
        }
    }
}

impl Token {
    pub fn priority(&self) -> i64 {
        match &self {
            Token::OPE(p) => p.priority(),
            Token::EQUAL => 1,
            _ => 0,
        }
    }
    pub fn get_text(&self) -> String {
        match &self {
            Token::IDENTIFIER(s) => s.clone(),
            _ => "".to_string(),
        }
    }
    pub fn to_token_type(&self) -> TokenType {
        match &self {
            Token::OPE(_) => TokenType::OPE,
            Token::IDENTIFIER(_) => TokenType::IDENTIFIER,
            Token::INT(_) => TokenType::INT,
            Token::FLOAT(_) => TokenType::FLOAT,
            Token::EQUAL => TokenType::EQUAL,
            Token::RPAR => TokenType::RPAR,
            Token::LPAR => TokenType::LPAR,
            Token::Null => TokenType::Null,
        }
    }
}
