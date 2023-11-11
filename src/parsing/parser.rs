use crate::lexing::token::Token;
use crate::parsing::ast::{Ast, Parameters, token_to_parameter};
use crate::parsing::ast::Ast::{Nil, Node};

fn push_value(ast: Ast, token: Token) -> Ast {
    let parameter = token_to_parameter(token);
    match ast.clone() {
        Nil => {
            match parameter {
                Parameters::Null => Nil,
                _ => Ast::new(parameter)
            }
        }
        Node { value: _v, left: l, right: r } => {
            match *l {
                Nil => ast.insert_left(Ast::new(parameter)),
                Node { .. } => {
                    match *r {
                        Nil => ast.insert_right(Ast::new(parameter)),
                        Node { .. } => {
                            Ast::new(parameter).insert_left(ast.clone())
                        }
                    }
                }
            }
        }
    }
}

fn push_operator(ast: Ast, token: Token) -> Ast {
    let parameter = token_to_parameter(token);
    match ast.clone() {
        Nil => {
            match parameter {
                Parameters::Null => Nil,
                _ => Ast::new(parameter)
            }
        }
        Node { value: v, left: l, right: r } => {
            Node {
                value: parameter,
                left: Box::from(Node { value: v, left: l, right: r }),
                right: Box::from(Nil),
            }
        }
    }
}


pub fn parse(lst: &Vec<Token>) -> Ast {
    fn aux(lst: &[Token], mut acc: Ast, _last_token: &Token) -> Ast {
        match lst {
            [] => acc,
            [Token::INT(i), q @ ..] => {
                acc = push_value(acc, Token::INT(*i));
                aux(q, acc, &Token::INT(*i))
            }
            [Token::FLOAT(f), q @ ..] => {
                acc = push_value(acc, Token::FLOAT(*f));
                aux(q, acc, &Token::FLOAT(*f))
            }
            [Token::IDENTIFIER(s), q @ ..] => {
                acc = push_value(acc, Token::IDENTIFIER(s.to_string()));
                aux(q, acc, &Token::IDENTIFIER(s.clone()))
            }
            [Token::OPE(p), q @ ..] => {
                acc = push_operator(acc, Token::OPE(p.clone()));
                aux(q, acc, &Token::OPE(p.clone()))
            }
            [Token::EQUAL, q @ ..] => {
                acc = push_operator(acc, Token::EQUAL);
                aux(q, acc, &Token::EQUAL)
            }
            [h, q @ ..] => aux(q, acc, h)
        }
    }

    aux(lst.as_slice(), Ast::Nil, &Token::Null)
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
}