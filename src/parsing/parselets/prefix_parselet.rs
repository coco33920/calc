use crate::lexing::token::Token;
use crate::parsing::ast::{Ast, Parameters};
use crate::parsing::parser::Parsing;

pub trait PrefixParselet {
    fn parse(&self, parser: &dyn Parsing, token: Token) -> Ast;
}

struct IdentifierParselet {}

struct OperatorPrefixParselet {}

impl PrefixParselet for IdentifierParselet {
    fn parse(&self, _parser: &dyn Parsing, token: Token) -> Ast {
        Ast::Node {
            value: Parameters::Identifier(token.get_text()),
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
