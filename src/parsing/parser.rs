use core::slice::Iter;
use std::collections::HashMap;

use crate::lexing::token::Token::*;
use crate::lexing::token::{Token, TokenType};
use crate::parsing::ast::Ast;
use crate::parsing::ast::Ast::Nil;
use crate::parsing::parselets::infix_parselet::InfixParselet;
use crate::parsing::parselets::prefix_parselet::PrefixParselet;

pub trait Parsing {
    fn parse(&self) -> Ast;
}

pub struct CalcParser<'a> {
    tokens: Iter<'a, Token>,
    read: Vec<Token>,
    infix_parselet: HashMap<TokenType, Box<dyn InfixParselet>>,
    prefix_parselet: HashMap<TokenType, Box<dyn PrefixParselet>>,
}

pub fn init_calc_parser(input: &Vec<Token>) -> CalcParser {
    CalcParser {
        tokens: input.iter(),
        read: Vec::new(),
        infix_parselet: HashMap::new(),
        prefix_parselet: HashMap::new(),
    }
}

impl CalcParser<'_> {
    fn init(self) -> Self {
        //TODO
        //init the parselets
        //
        //
        self
    }

    fn parse_expression(&mut self,precende: i64) -> Ast {
        let token = self.consume();
        let prefix = self.prefix_parselet.get(&token.to_token_type());
        let mut left = match prefix {
            None => Ast::Nil,
            Some(t) => {
                (*t).parse(self,token)
            }
        };
        while precende < self.get_precedence() {
            let t = self.consume();
            let infix = match self.infix_parselet.get(&(t.to_token_type())) {
                None => Ast::Nil,
                Some(t) => {
                    left = (*t).parse(self,&left,token.clone())
                }
            };
        }
        left
    }

    fn parse_expression_empty(&mut self) -> Ast {
        self.parse_expression(0)
    }

    fn look_ahead(&mut self, distance: usize) -> Token {
        while distance >= self.read.len() {
            match self.tokens.next() {
                None => break,
                Some(t) => self.read.push(t.clone()),
            }
        }
        match self.read.get(distance) {
            None => Token::Null,
            Some(t) => t.clone(),
        }
    }
    pub fn consume(&mut self) -> Token {
        self.look_ahead(0);
        self.read.remove(0)
    }
    pub fn consume_expected(&mut self, expected: TokenType) -> Token {
        self.look_ahead(0);
        match self.read.remove(0) {
            t => {
                if t.to_token_type() == expected {
                    t
                } else {
                    Token::Null
                }
            }
        }
    }

    fn match_token(&mut self, expected: TokenType) -> bool {
        let token = self.look_ahead(0);
        if token.to_token_type() != expected {
            return false;
        }
        return true;
    }

    fn get_precedence(&mut self) -> i64 {
        let parser: dyn InfixParselet = self
            .infix_parselet
            .get(&(self.look_ahead(0).to_token_type()));
        parser.get_precedence()
    }
}

impl Parsing for CalcParser<'_> {
    fn parse(&self) -> Ast {
        Nil
    }
}

#[cfg(test)]
mod test {
    use crate::lexing::lexer::lex;
    use crate::parsing::ast::{Ast, Parameters};
    use crate::parsing::parser::{init_calc_parser, CalcParser, Parsing};

    #[test]
    pub fn test_parse_nil() {
        let b = lex("".to_string());
        let parser: &CalcParser = &init_calc_parser(&b).init();
        let expected = Ast::Nil;
        let result = parser.parse();
        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_parse_one_token() {
        let b = lex("2".to_string());
        let parser: &CalcParser = &init_calc_parser(&b).init();
        let expected = Ast::Node {
            value: Parameters::Int(2),
            left: Box::new(Ast::Nil),
            right: Box::new(Ast::Nil),
        };

        let result = parser.parse();
        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_parse_plus_operation() {
        let b = lex("2+2".to_string());
        let parser: &CalcParser = &init_calc_parser(&b).init();
        let expected = Ast::Node {
            value: Parameters::PlusOperation,
            left: Box::new(Ast::new(Parameters::Int(2))),
            right: Box::new(Ast::new(Parameters::Int(2))),
        };
        let result = parser.parse();
        assert_eq!(result, expected)
    }

    #[test]
    pub fn test_parse_plus_operation_hard() {
        let b = lex("1+1+&".to_string());
        let parser: &CalcParser = &init_calc_parser(&b).init();
        let expected = Ast::Node {
            value: Parameters::PlusOperation,
            left: Box::from(Ast::Node {
                value: Parameters::PlusOperation,
                left: Box::from(Ast::new(Parameters::Int(1))),
                right: Box::from(Ast::new(Parameters::Int(1))),
            }),
            right: Box::from(Ast::new(Parameters::Int(1))),
        };
        let result = parser.parse();
        assert_eq!(result, expected)
    }

    #[test]
    pub fn test_parse_minus_operation() {
        let b = lex("2-2".to_string());
        let parser: &CalcParser = &init_calc_parser(&b).init();
        let expected = Ast::Node {
            value: Parameters::MinusOperation,
            left: Box::new(Ast::new(Parameters::Int(2))),
            right: Box::new(Ast::new(Parameters::Int(2))),
        };
        let result = parser.parse();
        assert_eq!(result, expected)
    }

    #[test]
    pub fn test_parse_mult_operation() {
        let b = lex("2*2".to_string());
        let parser: &CalcParser = &init_calc_parser(&b).init();
        let expected = Ast::Node {
            value: Parameters::MultiplicationOperation,
            left: Box::new(Ast::new(Parameters::Int(2))),
            right: Box::new(Ast::new(Parameters::Int(2))),
        };
        let result = parser.parse();
        assert_eq!(result, expected)
    }

    #[test]
    pub fn test_parse_hard_mult_operation() {
        let b = lex("2*2*2".to_string());
        let parser: &CalcParser = &init_calc_parser(&b).init();
        let expected = Ast::Node {
            value: Parameters::MultiplicationOperation,
            left: Box::from(Ast::Node {
                value: Parameters::MultiplicationOperation,
                left: Box::from(Ast::new(Parameters::Int(2))),
                right: Box::from(Ast::new(Parameters::Int(2))),
            }),
            right: Box::from(Ast::new(Parameters::Int(2))),
        };
        let result = parser.parse();
    }

    #[test]
    pub fn test_parse_divide_operation() {
        let b = lex("2/2".to_string());
        let parser: &CalcParser = &init_calc_parser(&b).init();
        let expected = Ast::Node {
            value: Parameters::DivideOperation,
            left: Box::new(Ast::new(Parameters::Int(2))),
            right: Box::new(Ast::new(Parameters::Int(2))),
        };
        let result = parser.parse();
        assert_eq!(result, expected)
    }

    #[test]
    pub fn test_assignment() {
        let b = lex("i=1".to_string());
        let parser: &CalcParser = &init_calc_parser(&b).init();
        let expected = Ast::Node {
            value: Parameters::Assign,
            left: Box::new(Ast::new(Parameters::Identifier("i".to_string()))),
            right: Box::new(Ast::new(Parameters::Int(1))),
        };
        let result = parser.parse();
        assert_eq!(result, expected);
    }

    #[test]
    pub fn simple_parenthesis() {
        let b = lex("1+(1*1)".to_string());
        let parser: &CalcParser = &init_calc_parser(&b).init();
        let expected = Ast::Node {
            value: Parameters::PlusOperation,
            left: Box::from(Ast::new(Parameters::Int(1))),
            right: Box::from(Ast::Node {
                value: Parameters::MultiplicationOperation,
                left: Box::new(Ast::new(Parameters::Int(1))),
                right: Box::new(Ast::new(Parameters::Int(1))),
            }),
        };
        let result = parser.parse();
        assert_eq!(result, expected)
    }

    #[test]
    pub fn hard_parenthesis() {
        let b = lex("1+(1*(1/1))".to_string());
        let parser: &CalcParser = &init_calc_parser(&b).init();
        let expected = Ast::Node {
            value: Parameters::PlusOperation,
            left: Box::from(Ast::new(Parameters::Int(1))),
            right: Box::from(Ast::Node {
                value: Parameters::DivideOperation,
                left: Box::from(Ast::Node {
                    value: Parameters::MultiplicationOperation,
                    left: Box::from(Ast::new(Parameters::Int(1))),
                    right: Box::from(Ast::new(Parameters::Int(1))),
                }),
                right: Box::from(Ast::new(Parameters::Int(1))),
            }),
        };
        let result = parser.parse();
        assert_eq!(result, expected)
    }

    #[test]
    pub fn without_parenthesis() {
        let b = lex("1+1*1".to_string());
        let parser: &CalcParser = &init_calc_parser(&b).init();
        let expected = Ast::Node {
            value: Parameters::PlusOperation,
            left: Box::from(Ast::new(Parameters::Int(1))),
            right: Box::from(Ast::Node {
                value: Parameters::MultiplicationOperation,
                left: Box::new(Ast::new(Parameters::Int(1))),
                right: Box::new(Ast::new(Parameters::Int(1))),
            }),
        };
        let result = parser.parse();
        assert_eq!(result, expected)
    }

    #[test]
    pub fn hard_without_parenthesis() {
        let b = lex("1+1*1/1".to_string());
        let parser: &CalcParser = &init_calc_parser(&b).init();
        let expected = Ast::Node {
            value: Parameters::PlusOperation,
            left: Box::from(Ast::new(Parameters::Int(1))),
            right: Box::from(Ast::Node {
                value: Parameters::MultiplicationOperation,
                left: Box::from(Ast::Node {
                    value: Parameters::DivideOperation,
                    left: Box::from(Ast::new(Parameters::Int(1))),
                    right: Box::from(Ast::new(Parameters::Int(1))),
                }),
                right: Box::from(Ast::new(Parameters::Int(1))),
            }),
        };
        let result = parser.parse();
        assert_eq!(result, expected)
    }
}
