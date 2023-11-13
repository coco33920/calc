use std::borrow::Borrow;

use crate::lexing::token::Token;
use crate::parsing::ast::{token_to_parameter, Ast, Parameters};
use crate::parsing::parser::CalcParser;

pub trait PrefixParselet {
    fn parse(&self, parser: &mut CalcParser, token: Token) -> Ast;
}

#[derive(Clone)]
pub struct IdentifierParselet {}

#[derive(Clone)]
pub struct OperatorPrefixParselet {}

#[derive(Clone)]
pub struct IntParselet {}

#[derive(Clone)]
pub struct FloatParselet {}

#[derive(Clone)]
pub struct NullParselet {}

impl PrefixParselet for IdentifierParselet {
    fn parse(&self, _parser: &mut CalcParser, token: Token) -> Ast {
        Ast::Node {
            value: Parameters::Identifier(token.get_text()),
            left: Box::from(Ast::Nil),
            right: Box::from(Ast::Nil),
        }
    }
}

impl PrefixParselet for IntParselet {
    fn parse(&self, _parser: &mut CalcParser, token: Token) -> Ast {
        Ast::Node {
            value: Parameters::Int(token.get_int()),
            left: Box::from(Ast::Nil),
            right: Box::from(Ast::Nil),
        }
    }
}

impl PrefixParselet for FloatParselet {
    fn parse(&self, _parser: &mut CalcParser, token: Token) -> Ast {
        Ast::Node {
            value: Parameters::Float(token.get_float()),
            left: Box::from(Ast::Nil),
            right: Box::from(Ast::Nil),
        }
    }
}

impl PrefixParselet for OperatorPrefixParselet {
    fn parse(&self, parser: &mut CalcParser, token: Token) -> Ast {
        let operand = parser.parse_expression_empty();
        Ast::Node {
            value: token_to_parameter(token),
            left: Box::from(operand),
            right: Box::from(Ast::Nil),
        }
    }
}

impl PrefixParselet for NullParselet {
    fn parse(&self, _parser: &mut CalcParser, token: Token) -> Ast {
        Ast::Nil
    }
}
