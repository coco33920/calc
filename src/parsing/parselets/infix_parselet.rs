use crate::lexing::token::{Precedence, Token};
use crate::parsing::ast::{Ast, Parameters};
use crate::parsing::parser::CalcParser;

pub trait InfixParselet {
    fn parse(&self, parser: &mut CalcParser, left: &Ast, token: Token) -> Ast;
    fn get_precedence(&self) -> i64;
}

pub struct PlusParselet {
    pub(crate) is_right: bool,
}

pub struct MinusParselet {
    pub(crate) is_right: bool,
}

pub struct MultParselet {
    pub(crate) is_right: bool,
}

pub struct DivideParselet {
    pub(crate) is_right: bool,
}

#[derive(Clone)]
pub struct AssignParselet {}

pub struct ExpoParselet {
    pub is_right: bool,
}

pub struct NullParset {}

impl InfixParselet for PlusParselet {
    fn parse(&self, parser: &mut CalcParser, left: &Ast, _token: Token) -> Ast {
        let right = parser.parse_expression(if self.is_right {
            self.get_precedence() - 1
        } else {
            self.get_precedence()
        });
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
    fn parse(&self, parser: &mut CalcParser, left: &Ast, _token: Token) -> Ast {
        let right = parser.parse_expression(if self.is_right {
            self.get_precedence() - 1
        } else {
            self.get_precedence()
        });
        Ast::Node {
            value: Parameters::MinusOperation,
            left: Box::new(left.clone()),
            right: Box::new(right),
        }
    }

    fn get_precedence(&self) -> i64 {
        Precedence::MINUS as i64
    }
}

impl InfixParselet for MultParselet {
    fn parse(&self, parser: &mut CalcParser, left: &Ast, _token: Token) -> Ast {
        let right = parser.parse_expression(if self.is_right {
            self.get_precedence() - 1
        } else {
            self.get_precedence()
        });
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
    fn parse(&self, parser: &mut CalcParser, left: &Ast, _token: Token) -> Ast {
        let right = parser.parse_expression(if self.is_right {
            self.get_precedence() - 1
        } else {
            self.get_precedence()
        });
        Ast::Node {
            value: Parameters::DivideOperation,
            left: Box::new(left.clone()),
            right: Box::new(right),
        }
    }

    fn get_precedence(&self) -> i64 {
        Precedence::DIVIDE as i64
    }
}

impl InfixParselet for ExpoParselet {
    fn parse(&self, parser: &mut CalcParser, left: &Ast, _token: Token) -> Ast {
        let right = parser.parse_expression(if self.is_right {
            self.get_precedence() - 1
        } else {
            self.get_precedence()
        });
        Ast::Node {
            value: Parameters::ExpoOperation,
            left: Box::new(left.clone()),
            right: Box::new(right),
        }
    }

    fn get_precedence(&self) -> i64 {
        Precedence::EXPONENT as i64
    }
}

impl InfixParselet for AssignParselet {
    fn parse(&self, parser: &mut CalcParser, left: &Ast, _token: Token) -> Ast {
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
    fn parse(&self, _parser: &mut CalcParser, left: &Ast, _token: Token) -> Ast {
        left.clone()
    }

    fn get_precedence(&self) -> i64 {
        0
    }
}
