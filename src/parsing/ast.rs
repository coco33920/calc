use crate::lexing::token::{Operator, Token};

#[derive(Debug)]
pub enum Parameters {
    Int(i64),
    Float(f64),
    Identifier(String),
    PlusOperation,
    MinusOperation,
    MultiplicationOperation,
    DivideOperation,
    Null,
}

#[derive(Debug)]
pub enum Ast {
    Nil,
    Node(Parameters,Box<Ast>,Box<Ast>)
}


pub fn token_to_parameter(token: Token) -> Parameters {
    match token {
        Token::INT(i) => Parameters::Int(i),
        Token::FLOAT(f) => Parameters::Float(f),
        Token::IDENTIFIER(s) => Parameters::Identifier(s),
        Token::OPE(Operator::PLUS) => Parameters::PlusOperation,
        Token::OPE(Operator::MINUS) => Parameters::MinusOperation,
        Token::OPE(Operator::MULTIPLICATION) => Parameters::MultiplicationOperation,
        Token::OPE(Operator::DIVIDE) => Parameters::DivideOperation,
        _ => Parameters::Null
    }
}
