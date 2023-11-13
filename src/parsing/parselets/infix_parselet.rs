use crate::lexing::token::Token;
use crate::parsing::ast::Ast;
use crate::parsing::parser::{Parsing};

pub trait InfixParselet {

    fn parse(&self,parser: &dyn Parsing, left: &Ast, token: Token) -> Ast;
    fn get_precedence(&self) -> i64;


}