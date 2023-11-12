use std::collections::HashMap;

use crate::interpreting::function::{add, assign, divide, minus, mult};
use crate::parsing::ast::{Ast, Parameters};

pub fn interpret(ast: Ast, mut ram: &mut HashMap<String, Parameters>) -> Parameters {
    match ast {
        Ast::Nil => Parameters::Null,
        Ast::Node {
            value: v,
            left: l,
            right: r,
        } => {
            let param1 = interpret(*l, &mut ram);
            let param2 = interpret(*r, &mut ram);
            match v {
                Parameters::PlusOperation => add(param1, param2, Some(&ram)),
                Parameters::MinusOperation => minus(param1, param2, Some(&ram)),
                Parameters::MultiplicationOperation => mult(param1, param2, Some(&ram)),
                Parameters::DivideOperation => divide(param1, param2, Some(&ram)),
                Parameters::Assign => {
                    let (a, b) = assign(param1, param2);
                    if a != "".to_string() {
                        (ram).insert(a, b);
                    }
                    Parameters::Null
                }
                Parameters::Float(f) => Parameters::Float(f),
                Parameters::Int(i) => Parameters::Int(i),
                Parameters::Identifier(s) => Parameters::Identifier(s),
                Parameters::Null => Parameters::Null,
            }
        }
    }
}
