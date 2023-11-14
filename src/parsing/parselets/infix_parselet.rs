use crate::lexing::token::{Precedence, Token, TokenType};
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

pub struct CallParselet {}

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

fn create_ast_from_lst(lst: &Vec<Ast>, name: String) -> Ast {
    fn aux(lst: &[Ast], mut acc: Ast, name: String) -> Ast {
        match lst {
            [] => acc,
            [h, q @ ..] => {
                acc = Ast::Node {
                    value: Parameters::Call(name.clone()),
                    left: Box::from(h.clone()),
                    right: Box::from(acc),
                };
                aux(q, acc, name.clone())
            }
        }
    }

    aux(lst.as_slice(), Ast::Nil, name)
}

impl InfixParselet for CallParselet {
    fn parse(&self, parser: &mut CalcParser, left: &Ast, token: Token) -> Ast {
        let name = match left {
            Ast::Nil => "",
            Ast::Node {
                value: v,
                left: _left,
                right: _right,
            } => match v {
                Parameters::Identifier(s) => s.as_str(),
                _ => "",
            },
        };

        let mut lst: Vec<Ast> = Vec::new();
        if !parser.match_token(TokenType::RPAR) {
            lst.push(parser.parse_expression_empty());
            while parser.match_token(TokenType::COMMA) {
                parser.consume();
                let ast = parser.parse_expression_empty();
                lst.push(ast);
            }
            parser.consume_expected(TokenType::RPAR);
        }
        let l = create_ast_from_lst(&lst, name.clone().to_string());
        l
    }

    fn get_precedence(&self) -> i64 {
        Precedence::CALL as i64
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
