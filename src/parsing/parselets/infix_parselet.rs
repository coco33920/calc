use crate::lexing::token::Token;
use crate::parsing::ast::Ast;
use crate::parsing::ast::Parameters::PlusOperation;
use crate::parsing::parser::Parsing;
use std::ops::Div;

pub trait InfixParselet {
    fn parse(&self, parser: &dyn Parsing, left: &Ast, token: Token) -> Ast;
    fn get_precedence(&self) -> i64;
}

pub struct PlusParselet {}
pub struct MinusParselet {}
pub struct MultParselet {}
pub struct DivideParselet {}

pub struct NullParset {}

impl InfixParselet for PlusParselet {
    fn parse(&self, parser: &dyn Parsing, left: &Ast, token: Token) -> Ast {
        Ast::Nil
    }

    fn get_precedence(&self) -> i64 {
        0
    }
}

impl InfixParselet for MinusParselet {
    fn parse(&self, parser: &dyn Parsing, left: &Ast, token: Token) -> Ast {
        Ast::Nil
    }

    fn get_precedence(&self) -> i64 {
        0
    }
}

impl InfixParselet for MultParselet {
    fn parse(&self, parser: &dyn Parsing, left: &Ast, token: Token) -> Ast {
        Ast::Nil
    }

    fn get_precedence(&self) -> i64 {
        0
    }
}

impl InfixParselet for DivideParselet {
    fn parse(&self, parser: &dyn Parsing, left: &Ast, token: Token) -> Ast {
        Ast::Nil
    }

    fn get_precedence(&self) -> i64 {
        0
    }
}

impl InfixParselet for NullParset {
    fn parse(&self, parser: &dyn Parsing, left: &Ast, token: Token) -> Ast {
        Ast::Nil
    }

    fn get_precedence(&self) -> i64 {
        0
    }
}