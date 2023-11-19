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

#[derive(Clone)]
pub struct VecParselet {}

#[derive(Clone)]
pub struct QuoteParselet {}

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

impl PrefixParselet for VecParselet {
    fn parse(&self, parser: &mut CalcParser, _token: Token) -> Ast {
        let mut vec: Vec<Ast> = Vec::new();

        if !parser.match_token(TokenType::RBRACKET) {
            vec.push(parser.parse_expression_empty());
            while parser.match_token(TokenType::COMMA) {
                parser.consume();
                vec.push(parser.parse_expression_empty());
            }
            parser.consume_expected(TokenType::RBRACKET);
        }

        Ast::Node {
            value: crate::parsing::ast::Parameters::Vector(Box::from(vec)),
            left: Box::new(Ast::Nil),
            right: Box::new(Ast::Nil),
        }
    }
}

impl PrefixParselet for QuoteParselet {
    fn parse(&self, parser: &mut CalcParser, _token: Token) -> Ast {
        let mut str: String = String::new();

        if !parser.match_token(TokenType::QUOTE) {
            while !parser.match_token(TokenType::QUOTE) {
                match parser.consume() {
                    Token::IDENTIFIER(s) => str = str + &s.to_string(),

                    t => str = str + &t.to_string(),
                }
            }
            parser.consume_expected(TokenType::QUOTE);
        }

        Ast::Node {
            value: crate::parsing::ast::Parameters::Str(str.trim().to_string()),
            left: Box::new(Ast::Nil),
            right: Box::new(Ast::Nil),
        }
    }
}
