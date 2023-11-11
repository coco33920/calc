use crate::lexing::token::{Operator, Token};
use crate::parsing::ast::Ast::{Nil, Node};
use crate::parsing::ast::Parameters::*;

#[derive(Debug)]
pub enum Parameters {
    Int(i64),
    Float(f64),
    Identifier(String),
    PlusOperation,
    MinusOperation,
    MultiplicationOperation,
    DivideOperation,
    Assign,
    Null,
}

#[derive(Debug)]
pub enum Ast {
    Nil,
    Node {
        value: Parameters,
        left: Box<Ast>,
        right: Box<Ast>,
    },
}

impl Clone for Parameters {
    fn clone(&self) -> Self {
        match self {
            Int(i) => Int(*i),
            Float(f) => Float(*f),
            Identifier(s) => Identifier(s.to_string().clone()),
            PlusOperation => PlusOperation,
            MinusOperation => MinusOperation,
            MultiplicationOperation => MultiplicationOperation,
            DivideOperation => DivideOperation,
            Assign => Assign,
            Null => Null
        }
    }
}

impl Clone for Ast {
    fn clone(&self) -> Self {
        match self {
            Nil => Nil,
            Node { value: v, left: l, right: r } => {
                Node { value: v.clone(), left: l.clone(), right: r.clone() }
            }
        }
    }
}

impl Ast {
    pub fn new(p: Parameters) -> Self {
        Node {
            value: p,
            left: Box::from(Nil),
            right: Box::from(Nil),
        }
    }
    pub fn insert_left(self, node: Ast) -> Self {
        match &self {
            Nil => node,
            Node { value, left: _left, right } => {
                Node {
                    value: value.clone(),
                    left: Box::from(node),
                    right: right.clone(),
                }
            }
        }
    }
    pub fn insert_right(self, node: Ast) -> Self {
        match &self {
            Nil => node,
            Node { value, left, right: _right } => {
                Node {
                    value: value.clone(),
                    left: left.clone(),
                    right: Box::from(node),
                }
            }
        }
    }
}

impl PartialEq for Parameters {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Int(i), Int(i2)) => i == i2,
            (Float(f), Float(f2)) => f == f2,
            (Identifier(s), Identifier(s2)) => s == s2,
            (PlusOperation, PlusOperation) => true,
            (MinusOperation, MinusOperation) => true,
            (MultiplicationOperation, MultiplicationOperation) => true,
            (DivideOperation, DivideOperation) => true,
            (Assign,Assign) => true,
            _ => false
        }
    }

    fn ne(&self, other: &Self) -> bool {
        if self == other {
            false
        } else {
            true
        }
    }
}

impl PartialEq for Ast {
    fn eq(&self, other: &Self) -> bool {
        match (&self, &other) {
            (Nil, Nil) => true,
            (Node { value: p, left: l, right: r }, Node { value: p2, left: l2, right: r2 }) => {
                if p != p2 {
                    return false;
                }
                return (l.eq(l2)) && (r.eq(r2));
            }
            _ => false
        }
    }

    fn ne(&self, other: &Self) -> bool {
        if self == other {
            false
        } else {
            true
        }
    }
}


pub fn token_to_parameter(token: Token) -> Parameters {
    match token {
        Token::INT(i) => Int(i),
        Token::FLOAT(f) => Float(f),
        Token::IDENTIFIER(s) => Identifier(s),
        Token::OPE(Operator::PLUS) => PlusOperation,
        Token::OPE(Operator::MINUS) => MinusOperation,
        Token::OPE(Operator::MULTIPLICATION) => MultiplicationOperation,
        Token::OPE(Operator::DIVIDE) => DivideOperation,
        Token::EQUAL => Assign,
        _ => Null
    }
}

#[cfg(test)]
mod test {
    use crate::parsing::ast::{Ast, Parameters};

    #[test]
    pub fn test_new() {
        let expected = Ast::Node { value: Parameters::Int(2), left: Box::from(Ast::Nil), right: Box::from(Ast::Nil) };
        let result = Ast::new(Parameters::Int(2));
        assert_eq!(result, expected)
    }

    #[test]
    pub fn test_insert_left() {
        let expected = Ast::Node {
            value: Parameters::Int(2),
            left: Box::from(Ast::new(Parameters::Int(2))),
            right: Box::from(Ast::Nil),
        };
        let result = Ast::new(Parameters::Int(2)).insert_left(Ast::new(Parameters::Int(2)));
        assert_eq!(result, expected)
    }

    #[test]
    pub fn test_insert_right() {
        let expected = Ast::Node {
            value: Parameters::Int(2),
            left: Box::from(Ast::Nil),
            right: Box::from(Ast::new(Parameters::Int(2))),
        };
        let result = Ast::new(Parameters::Int(2)).insert_right(Ast::new(Parameters::Int(2)));
        assert_eq!(result, expected)
    }

    #[test]
    pub fn test_insert_both() {
        let expected = Ast::Node {
            value: Parameters::Int(2),
            left: Box::from(Ast::new(Parameters::Int(2))),
            right: Box::from(Ast::new(Parameters::Int(2))),
        };
        let result = Ast::new(Parameters::Int(2))
            .insert_right(Ast::new(Parameters::Int(2)))
            .insert_left(Ast::new(Parameters::Int(2)));
        assert_eq!(result, expected);
    }
}
