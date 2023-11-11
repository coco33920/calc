use crate::lexing::token::Token;
use crate::lexing::token::Token::{LPAR, RPAR};
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
                        _ => ast.insert_right(Ast::new(parameter)),
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

fn push_ast(ast: Ast, ast2: Ast) -> Ast {
    match ast.clone() {
        Nil => ast2,
        Node { value: v, left: l, right: r } => {
            Node {
                value: v,
                left: Box::from(Node { value: l.clone().value(), left: Box::from(l.left()), right: r }),
                right: Box::from(ast2),
            }
        }
    }
}


pub fn parse(lst: &Vec<Token>) -> Ast {
    fn aux(lst: &[Token], mut acc: Ast, _last_operation: &Token) -> (Ast, Vec<Token>) {
        match lst {
            [] => (acc, Vec::new()),
            [Token::INT(i), q @ ..] => {
                acc = push_value(acc, Token::INT(*i));
                aux(q, acc, _last_operation)
            }
            [Token::FLOAT(f), q @ ..] => {
                acc = push_value(acc, Token::FLOAT(*f));
                aux(q, acc, _last_operation)
            }
            [Token::IDENTIFIER(s), q @ ..] => {
                acc = push_value(acc, Token::IDENTIFIER(s.to_string()));
                aux(q, acc, _last_operation)
            }
            [Token::OPE(p), q @ ..] => {
                acc = push_operator(acc, Token::OPE(p.clone()));
                aux(q, acc, &Token::OPE(p.clone()))
            }
            [Token::EQUAL, q @ ..] => {
                acc = push_operator(acc, Token::EQUAL);
                aux(q, acc, _last_operation)
            }
            [Token::LPAR, q @ ..] => {
                let (ac, rest) = aux(q, Nil, &Token::Null);
                acc = push_ast(acc, ac);
                aux(rest.as_slice(), acc, _last_operation)
            }
            [Token::RPAR, q @ ..] => {
                (acc, q.to_vec())
            }
            [h, q @ ..] => aux(q, acc, h)
        }
    }

    let (a, _) = aux(add_parenthesis(lst).as_slice(), Nil, &Token::Null);
    a
}


pub fn add_parenthesis(lst: &Vec<Token>) -> Vec<Token> {
    fn aux(lst: &[Token], mut acc: Vec<Token>, mut last_operation: Token, mut position_before_operation: usize, mut position: usize, mut add: bool, mut add2: bool, mut par: i32) -> (Vec<Token>, i32) {
        match lst {
            [] => (acc, par),
            [h, q @ ..] => {
                match h {
                    Token::OPE(p) => {
                        let precedence = last_operation.priority(&Token::OPE(p.clone()));
                        if last_operation == Token::OPE(p.clone()) {
                            acc.insert(position_before_operation, LPAR);
                            acc.push(Token::OPE(p.clone()));
                            acc.push(LPAR);
                            par += 2;
                            add2 = true;
                        } else {
                            if !precedence {
                                acc.push(Token::OPE(p.clone()));
                                last_operation = Token::OPE(p.clone());
                                position_before_operation = position;
                            } else {
                                println!("{:?} {:?}", acc, last_operation);
                                acc.insert(position_before_operation + 1, LPAR);
                                acc.push(Token::OPE(p.clone()));
                                last_operation = Token::OPE(p.clone());
                                par += 1;
                                position_before_operation = position;
                                add = true;
                            }
                        }
                    }
                    q => {
                        acc.push(q.clone());
                        if add {
                            acc.push(Token::RPAR);
                            add = false;
                            par -= 1;
                        }
                        if add2 {
                            acc.push(Token::RPAR);
                            add2 = false;
                            par -= 1;
                        }
                    }
                }

                position += 1;
                aux(q, acc, last_operation.clone(), position_before_operation, position, add, add2, par)
            }
        }
    }
    let (a, b) = aux(lst.as_slice(), Vec::new(), Token::Null, 0, 0, false, false, 0);
    let mut vec = a.clone();
    for _ in 0..b {
        vec.push(RPAR);
    }
    vec
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
                value: Parameters::MultiplicationOperation,
                left: Box::from(Ast::new(Parameters::Int(1))),
                right: Box::from(Ast::Node {
                    value: Parameters::DivideOperation,
                    left: Box::from(Ast::new(Parameters::Int(1))),
                    right: Box::from(Ast::new(Parameters::Int(1))),
                }),
            }),
        };
        let result = parse(&lex("1+(1*(1/1))".to_string()));
    }
}