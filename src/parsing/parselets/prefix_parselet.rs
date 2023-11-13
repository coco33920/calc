use crate::lexing::token::Token;
use crate::parsing::ast::{Ast, Parameters};
use crate::parsing::parser::Parsing;
use std::borrow::Borrow;

pub trait PrefixParselet {
    fn parse(&self, parser: &dyn Parsing, token: Token) -> Ast;
}

#[derive(Clone)]
pub struct IdentifierParselet {}
#[derive(Clone)]
pub struct OperatorPrefixParselet {}

#[derive(Clone)]
pub struct IntParselet {}

#[derive(Clone)]
pub struct FloatParselet {}

impl PrefixParselet for IdentifierParselet {
    fn parse(&self, _parser: &dyn Parsing, token: Token) -> Ast {
        Ast::Node {
            value: Parameters::Identifier(token.get_text()),
            left: Box::from(Ast::Nil),
            right: Box::from(Ast::Nil),
        }
    }
}

impl PrefixParselet for IntParselet {
    fn parse(&self, _parser: &dyn Parsing, token: Token) -> Ast {
        Ast::Node {
            value: Parameters::Int(token.get_int()),
            left: Box::from(Ast::Nil),
            right: Box::from(Ast::Nil),
        }
    }
}

impl PrefixParselet for FloatParselet {
    fn parse(&self, _parser: &dyn Parsing, token: Token) -> Ast {
        Ast::Node {
            value: Parameters::Float(token.get_float()),
            left: Box::from(Ast::Nil),
            right: Box::from(Ast::Nil),
        }
    }
}

impl PrefixParselet for OperatorPrefixParselet {
    fn parse(&self, parser: &dyn Parsing, token: Token) -> Ast {
        Ast::Nil
    }
}
