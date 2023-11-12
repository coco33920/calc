use crate::lexing::token::Token;
use crate::lexing::token::Token::*;
use crate::parsing::ast::Ast::{Nil};
use crate::parsing::ast::{Ast};

pub fn parse(lst: &Vec<Token>) -> Ast {
    Nil
}

#[cfg(test)]
mod test {
    use crate::lexing::lexer::lex;
    use crate::parsing::ast::{Ast, Parameters};
    use crate::parsing::parser::parse;

    #[test]
    pub fn test_parse_one_token() {
        let expected = Ast::Node {
            value: Parameters::Int(2),
            left: Box::new(Ast::Nil),
            right: Box::new(Ast::Nil),
        };

        let result = parse(&lex("2".to_string()));
        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_parse_plus_operation() {
        let expected = Ast::Node {
            value: Parameters::PlusOperation,
            left: Box::new(Ast::new(Parameters::Int(2))),
            right: Box::new(Ast::new(Parameters::Int(2))),
        };
        let result = parse(&lex("2+2".to_string()));
        assert_eq!(result, expected)
    }

    #[test]
    pub fn test_parse_minus_operation() {
        let expected = Ast::Node {
            value: Parameters::MinusOperation,
            left: Box::new(Ast::new(Parameters::Int(2))),
            right: Box::new(Ast::new(Parameters::Int(2))),
        };
        let result = parse(&lex("2-2".to_string()));
        assert_eq!(result, expected)
    }

    #[test]
    pub fn test_parse_mult_operation() {
        let expected = Ast::Node {
            value: Parameters::MultiplicationOperation,
            left: Box::new(Ast::new(Parameters::Int(2))),
            right: Box::new(Ast::new(Parameters::Int(2))),
        };
        let result = parse(&lex("2*2".to_string()));
        assert_eq!(result, expected)
    }

    #[test]
    pub fn test_parse_divide_operation() {
        let expected = Ast::Node {
            value: Parameters::DivideOperation,
            left: Box::new(Ast::new(Parameters::Int(2))),
            right: Box::new(Ast::new(Parameters::Int(2))),
        };
        let result = parse(&lex("2/2".to_string()));
        assert_eq!(result, expected)
    }

    #[test]
    pub fn test_assignment() {
        let expected = Ast::Node {
            value: Parameters::Assign,
            left: Box::new(Ast::new(Parameters::Identifier("i".to_string()))),
            right: Box::new(Ast::new(Parameters::Int(1))),
        };
        let result = parse(&lex("i=1".to_string()));
        assert_eq!(result, expected);
    }

    #[test]
    pub fn simple_parenthesis() {
        let expected = Ast::Node {
            value: Parameters::PlusOperation,
            left: Box::from(Ast::new(Parameters::Int(1))),
            right: Box::from(Ast::Node {
                value: Parameters::MultiplicationOperation,
                left: Box::new(Ast::new(Parameters::Int(1))),
                right: Box::new(Ast::new(Parameters::Int(1))),
            }),
        };
        let result = parse(&lex("1+(1*1)".to_string()));
        assert_eq!(result, expected)
    }

    #[test]
    pub fn hard_parenthesis() {
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
        let result = parse(&lex("1+(1*(1/1))".to_string()));
        assert_eq!(result, expected)
    }

    #[test]
    pub fn without_parenthesis() {
        let expected = Ast::Node {
            value: Parameters::PlusOperation,
            left: Box::from(Ast::new(Parameters::Int(1))),
            right: Box::from(Ast::Node {
                value: Parameters::MultiplicationOperation,
                left: Box::new(Ast::new(Parameters::Int(1))),
                right: Box::new(Ast::new(Parameters::Int(1))),
            }),
        };
        let result = parse(&lex("1+1*1".to_string()));
        assert_eq!(result, expected)
    }

    #[test]
    pub fn without_parenthesis_hard() {
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
        let result = parse(&lex("1+1*(1/1)".to_string()));
        assert_eq!(result, expected)
    }
}
