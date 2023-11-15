use crate::lexing::token::{Token, TokenType};
use crate::parsing::ast::{token_to_parameter, Ast};
use crate::parsing::parser::CalcParser;

pub trait PrefixParselet {
    fn parse(&self, parser: &mut CalcParser, token: Token) -> Ast;
}

#[derive(Clone)]
pub struct ValueParselet {}

#[derive(Clone)]
pub struct OperatorPrefixParselet {}

#[derive(Clone)]
pub struct NullParselet {}

#[derive(Clone)]
pub struct GroupParselet {}

impl PrefixParselet for ValueParselet {
    fn parse(&self, _parser: &mut CalcParser, token: Token) -> Ast {
        Ast::Node {
            value: token_to_parameter(token),
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
    fn parse(&self, _parser: &mut CalcParser, _token: Token) -> Ast {
        Ast::Nil
    }
}

impl PrefixParselet for GroupParselet {
    fn parse(&self, parser: &mut CalcParser, _token: Token) -> Ast {
        let expression = parser.parse_expression_empty();
        parser.consume_expected(TokenType::RPAR);
        expression
    }
}
