use crate::lexing::token::Token;
use crate::lexing::token::Token::*;
use crate::parsing::ast::Ast::{Nil, Node};
use crate::parsing::ast::{token_to_parameter, Ast, Parameters};

fn push_value(ast: Ast, token: Token) -> Ast {
    let parameter = token_to_parameter(token);
    match ast.clone() {
        Nil => match parameter {
            Parameters::Null => Nil,
            _ => Ast::new(parameter),
        },
        Node {
            value: _v,
            left: l,
            right: r,
        } => match *l {
            Nil => ast.insert_left(Ast::new(parameter)),
            Node { .. } => match *r {
                _ => ast.insert_right(Ast::new(parameter)),
            },
        },
    }
}

fn push_operator(ast: Ast, token: Token) -> Ast {
    let parameter = token_to_parameter(token);
    match ast.clone() {
        Nil => match parameter {
            Parameters::Null => Nil,
            _ => Ast::new(parameter),
        },
        Node {
            value: v,
            left: l,
            right: r,
        } => Node {
            value: parameter,
            left: Box::from(Node {
                value: v,
                left: l,
                right: r,
            }),
            right: Box::from(Nil),
        },
    }
}

fn push_ast(ast: Ast, ast2: Ast) -> Ast {
    match ast.clone() {
        Nil => ast2,
        Node {
            value: v,
            left: l,
            right: r,
        } => match *l {
            Nil => ast.clone().insert_left(ast2),
            Node {
                value: v1,
                left: l1,
                right: r1,
            } => match *r {
                Nil => ast.clone().insert_right(ast2),
                Node {
                    value: v2,
                    left: l2,
                    right: r2,
                } => push_ast(
                    Ast::new(v)
                        .insert_left(Ast::new(v1).insert_left(*l1).insert_right(*r1))
                        .insert_right(Ast::new(v2).insert_left(*l2).insert_right(*r2)),
                    ast2,
                ),
            },
        },
    }
}

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
