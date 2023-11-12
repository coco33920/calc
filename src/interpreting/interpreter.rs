use std::collections::HashMap;

use crate::interpreting::function::{add, assign, divide, minus, mult};
use crate::parsing::ast::{Ast, Parameters};

pub fn interpret(ast: Ast, ram: &HashMap<String, Parameters>) -> Parameters {
    match ast {
        Ast::Nil => Parameters::Null,
        Ast::Node {
            value: v,
            left: l,
            right: r,
        } => {
            let param1 = interpret(*l, ram);
            let param2 = interpret(*r, ram);
            match v {
                Parameters::PlusOperation => add(param1, param2),
                Parameters::MinusOperation => minus(param1, param2),
                Parameters::MultiplicationOperation => mult(param1, param2),
                Parameters::DivideOperation => divide(param1, param2),
                Parameters::Assign => {
                    assign(ram, param1, param2);
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
