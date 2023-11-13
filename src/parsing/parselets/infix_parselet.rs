use crate::lexing::token::{Precedence, Token};
use crate::parsing::ast::{Ast, Parameters};
use crate::parsing::parser::CalcParser;

pub trait InfixParselet {
    fn parse(&self, parser: &mut CalcParser, left: &Ast, token: Token) -> Ast;
    fn get_precedence(&self) -> i64;
}

pub struct PlusParselet {}

pub struct MinusParselet {}

pub struct MultParselet {}

pub struct DivideParselet {}

#[derive(Clone)]
pub struct AssignParselet {}

pub struct NullParset {}

impl InfixParselet for PlusParselet {
    fn parse(&self, parser: &mut CalcParser, left: &Ast, token: Token) -> Ast {
        let right = parser.parse_expression_empty();
        Ast::Node {
            value: Parameters::PlusOperation,
            left: Box::new(left.clone()),
            right: Box::new(right),
        }
    }

    fn get_precedence(&self) -> i64 {
        Precedence::SUM as i64
    }
}

impl InfixParselet for MinusParselet {
    fn parse(&self, parser: &mut CalcParser, left: &Ast, token: Token) -> Ast {
        let right = parser.parse_expression_empty();
        Ast::Node {
            value: Parameters::MinusOperation,
            left: Box::new(left.clone()),
            right: Box::new(right),
        }
    }

    fn get_precedence(&self) -> i64 {
        Precedence::SUM as i64
    }
}

impl InfixParselet for MultParselet {
    fn parse(&self, parser: &mut CalcParser, left: &Ast, token: Token) -> Ast {
        let right = parser.parse_expression_empty();
        Ast::Node {
            value: Parameters::MultiplicationOperation,
            left: Box::new(left.clone()),
            right: Box::new(right),
        }
    }

    fn get_precedence(&self) -> i64 {
        Precedence::PRODUCT as i64
    }
}

impl InfixParselet for DivideParselet {
    fn parse(&self, parser: &mut CalcParser, left: &Ast, token: Token) -> Ast {
        let right = parser.parse_expression_empty();
        Ast::Node {
            value: Parameters::DivideOperation,
            left: Box::new(left.clone()),
            right: Box::new(right),
        }
    }

    fn get_precedence(&self) -> i64 {
        Precedence::PRODUCT as i64
    }
}

impl InfixParselet for AssignParselet {
    fn parse(&self, parser: &mut CalcParser, left: &Ast, token: Token) -> Ast {
        let right = parser.parse_expression_empty();
        Ast::Node {
            value: Parameters::Assign,
            left: Box::new(left.clone()),
            right: Box::new(right),
        }
    }

    fn get_precedence(&self) -> i64 {
        Precedence::ASSIGNMENT as i64
    }
}

impl InfixParselet for NullParset {
    fn parse(&self, parser: &mut CalcParser, left: &Ast, token: Token) -> Ast {
        left.clone()
    }

    fn get_precedence(&self) -> i64 {
        0
    }
}
