use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    PLUS,
    MINUS,
    MULTIPLICATION,
    DIVIDE,
    EXPO,
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
    COMMA,
    Null,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum TokenType {
    PLUS,
    MINUS,
    MULTIPLICATION,
    DIVIDE,
    IDENTIFIER,
    INT,
    FLOAT,
    EQUAL,
    RPAR,
    LPAR,
    Null,
    COMMA,
    EXPO,
}

pub enum Precedence {
    ASSIGNMENT = 1,
    //CONDITIONAL = 2,
    SUM = 4,
    MINUS = 3,
    PRODUCT = 6,
    DIVIDE = 5,
    EXPONENT = 7,
    //PREFIX = 8,
    //POSTFIX = 9,
    CALL = 10,
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::PLUS => write!(f, "+"),
            Operator::MINUS => write!(f, "-"),
            Operator::DIVIDE => write!(f, "/"),
            Operator::MULTIPLICATION => write!(f, "*"),
            Operator::EXPO => write!(f, "^"),
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
            Token::COMMA => write!(f, ","),
            Token::Null => write!(f, "Null"),
        }
    }
}

impl Token {
    pub fn get_text(&self) -> String {
        match &self {
            Token::IDENTIFIER(s) => s.clone(),
            _ => "".to_string(),
        }
    }
    pub fn get_int(&self) -> i64 {
        match &self {
            Token::INT(i) => *i,
            _ => 0,
        }
    }
    pub fn get_float(&self) -> f64 {
        match &self {
            Token::FLOAT(f) => *f,
            _ => 0.0,
        }
    }
    pub fn to_token_type(&self) -> TokenType {
        match &self {
            Token::OPE(p) => match p {
                Operator::PLUS => TokenType::PLUS,
                Operator::MINUS => TokenType::MINUS,
                Operator::MULTIPLICATION => TokenType::MULTIPLICATION,
                Operator::DIVIDE => TokenType::DIVIDE,
                Operator::EXPO => TokenType::EXPO,
            },
            Token::IDENTIFIER(_) => TokenType::IDENTIFIER,
            Token::INT(_) => TokenType::INT,
            Token::FLOAT(_) => TokenType::FLOAT,
            Token::EQUAL => TokenType::EQUAL,
            Token::RPAR => TokenType::RPAR,
            Token::LPAR => TokenType::LPAR,
            Token::COMMA => TokenType::COMMA,
            Token::Null => TokenType::Null,
        }
    }
}
