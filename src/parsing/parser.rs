use crate::lexing::token::Token;
use crate::parsing::ast::{Ast, Parameters, token_to_parameter};

fn push_value(ast: Ast, token: Token) -> Ast {
    match ast {
        Ast::Nil => {
            let parameter = token_to_parameter(token);
            match parameter {
                Parameters::Null => Ast::Nil,
                _ => Ast::new(parameter)
            }
        },
        _ => Ast::Nil
    }
}


pub fn parse(lst: &Vec<Token>) -> Ast {
    fn aux(lst: &[Token], mut acc: Ast, last_token: &Token) -> Ast {
        match lst {
            [] => acc,
            [Token::INT(i), q @ ..] => {
                acc = push_value(acc, Token::INT(*i));
                aux(q, acc, last_token)
            },
            [Token::FLOAT(f), q @ ..] => {
                acc = push_value(acc, Token::FLOAT(*f));
                aux(q, acc, last_token)
            },
            [Token::IDENTIFIER(s), q @ ..] => {
                acc = push_value(acc, Token::IDENTIFIER(s.to_string()));
                aux(q, acc, last_token)
            },
            [h, q @ ..] => aux(q, acc, h)
        }
    }

    aux(lst.as_slice(), Ast::Nil, &Token::Null)
}