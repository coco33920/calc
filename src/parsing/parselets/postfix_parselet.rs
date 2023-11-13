use crate::lexing::token::Token;
use crate::parsing::ast::Ast;
use crate::parsing::parser::CalcParser;

pub trait PostfixParselet {
    fn parse(&self, parser: &mut CalcParser, left: &Ast, token: Token) -> Ast;
    fn get_precedence(&self) -> i64;
}
