use core::slice::Iter;

use crate::lexing::token::Token::*;
use crate::lexing::token::{Token, TokenType};
use crate::parsing::ast::Ast;
use crate::parsing::parselets::infix_parselet::{
    DivideParselet, InfixParselet, MinusParselet, MultParselet, PlusParselet,
};
use crate::parsing::parselets::prefix_parselet::{
    FloatParselet, IdentifierParselet, IntParselet, OperatorPrefixParselet, PrefixParselet,
};

#[derive(Clone)]
pub struct CalcParser<'a> {
    tokens: Iter<'a, Token>,
    read: Vec<Token>,
}

pub fn init_calc_parser(input: &Vec<Token>) -> CalcParser {
    CalcParser {
        tokens: input.iter(),
        read: Vec::new(),
    }
}

impl CalcParser<'_> {
    pub fn parse(&mut self) -> Ast {
        self.parse_expression_empty()
    }
    pub fn parse_expression(&mut self, precedence: i64) -> Ast {
        let token = self.consume();
        let prefix = self
            .clone()
            .get_prefix_parselet(token.clone().to_token_type());
        let mut left = match prefix {
            None => Ast::Nil,
            Some(t) => (*t).parse(self, token.clone()),
        };
        while precedence < self.get_precedence() {
            let t = self.consume();
            let infix = match self.clone().get_infix_parselet(t.clone().to_token_type()) {
                None => (),
                Some(t) => left = (*t).parse(self, &left, token.clone()),
            };
        }
        left
    }

    pub fn parse_expression_empty(&mut self) -> Ast {
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
            None => Null,
            Some(t) => t.clone(),
        }
    }
    pub fn consume(&mut self) -> Token {
        self.look_ahead(0);
        if self.read.len() == 0 {
            return Null;
        }
        self.read.remove(0)
    }

    pub fn consume_expected(&mut self, expected: TokenType) -> Token {
        self.look_ahead(0);
        match self.read.remove(0) {
            t => {
                if t.to_token_type() == expected {
                    t
                } else {
                    Null
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
        let p: Option<Box<dyn InfixParselet>> = self
            .clone()
            .get_infix_parselet(self.look_ahead(0).to_token_type());
        match p {
            None => 0,
            Some(t) => (*t).get_precedence(),
        }
    }

    pub fn get_infix_parselet(self, token_type: TokenType) -> Option<Box<dyn InfixParselet>> {
        match token_type {
            TokenType::PLUS => Some(Box::from(PlusParselet {})),
            TokenType::MINUS => Some(Box::from(MinusParselet {})),
            TokenType::MULTIPLICATION => Some(Box::from(MultParselet {})),
            TokenType::DIVIDE => Some(Box::from(DivideParselet {})),
            _ => None,
        }
    }

    pub fn get_prefix_parselet(self, token_type: TokenType) -> Option<Box<dyn PrefixParselet>> {
        match token_type {
            TokenType::PLUS => Some(Box::from(OperatorPrefixParselet {})),
            TokenType::MINUS => Some(Box::from(OperatorPrefixParselet {})),
            TokenType::MULTIPLICATION => Some(Box::from(OperatorPrefixParselet {})),
            TokenType::DIVIDE => Some(Box::from(OperatorPrefixParselet {})),
            TokenType::IDENTIFIER => Some(Box::from(IdentifierParselet {})),
            TokenType::INT => Some(Box::from(IntParselet {})),
            TokenType::FLOAT => Some(Box::from(FloatParselet {})),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::lexing::lexer::lex;
    use crate::parsing::ast::{Ast, Parameters};
    use crate::parsing::parser::{init_calc_parser, CalcParser};

    #[test]
    pub fn test_parse_nil() {
        let b = lex("".to_string());
        let mut parser: &mut CalcParser = &mut init_calc_parser(&b);
        let expected = Ast::Nil;
        let result = parser.parse();
        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_parse_one_token() {
        let b = lex("2".to_string());
        let mut parser: &mut CalcParser = &mut init_calc_parser(&b);
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
        let mut parser: &mut CalcParser = &mut init_calc_parser(&b);
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
        let mut parser: &mut CalcParser = &mut init_calc_parser(&b);
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
        let mut parser: &mut CalcParser = &mut init_calc_parser(&b);
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
        let mut parser: &mut CalcParser = &mut init_calc_parser(&b);
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
        let mut parser: &mut CalcParser = &mut init_calc_parser(&b);
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
        let mut parser: &mut CalcParser = &mut init_calc_parser(&b);
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
        let mut parser: &mut CalcParser = &mut init_calc_parser(&b);
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
        let mut parser: &mut CalcParser = &mut init_calc_parser(&b);
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
        let mut parser: &mut CalcParser = &mut init_calc_parser(&b);
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
        let mut parser: &mut CalcParser = &mut init_calc_parser(&b);
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
        let mut parser: &mut CalcParser = &mut init_calc_parser(&b);
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
