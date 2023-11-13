use crate::lexing::token::Token;
use crate::parsing::ast::Ast;
use crate::parsing::parser::CalcParser;

pub trait InfixParselet {
    fn parse(&self, parser: &CalcParser, left: &Ast, token: Token) -> Ast;
    fn get_precedence(&self) -> i64;
}

pub struct PlusParselet {}

pub struct MinusParselet {}

pub struct MultParselet {}

pub struct DivideParselet {}

pub struct NullParset {}

impl InfixParselet for PlusParselet {
    fn parse(&self, parser: &CalcParser, left: &Ast, token: Token) -> Ast {
        left.clone()
    }

    fn get_precedence(&self) -> i64 {
        0
    }
}

impl InfixParselet for MinusParselet {
    fn parse(&self, parser: &CalcParser, left: &Ast, token: Token) -> Ast {
        left.clone()
    }

    fn get_precedence(&self) -> i64 {
        0
    }
}

impl InfixParselet for MultParselet {
    fn parse(&self, parser: &CalcParser, left: &Ast, token: Token) -> Ast {
        left.clone()
    }

    fn get_precedence(&self) -> i64 {
        0
    }
}

impl InfixParselet for DivideParselet {
    fn parse(&self, parser: &CalcParser, left: &Ast, token: Token) -> Ast {
        left.clone()
    }

    fn get_precedence(&self) -> i64 {
        0
    }
}

impl InfixParselet for NullParset {
    fn parse(&self, parser: &CalcParser, left: &Ast, token: Token) -> Ast {
        left.clone()
    }

    fn get_precedence(&self) -> i64 {
        0
    }
}
