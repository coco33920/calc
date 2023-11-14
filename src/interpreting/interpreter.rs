use std::collections::HashMap;

use crate::interpreting::function::{add, assign, divide, expo, minus, mult};
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
                Parameters::ExpoOperation => expo(param1, param2, Some(&ram)),
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

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::interpreting::interpreter::interpret;
    use crate::parsing::ast::{Ast, Parameters};

    #[test]
    fn test_interpreter_int() {
        let mut ram: HashMap<String, Parameters> = HashMap::new();
        let expected = Parameters::Int(2);
        let ast = Ast::Node {
            value: Parameters::Int(2),
            left: Box::from(Ast::Nil),
            right: Box::from(Ast::Nil),
        };
        let result = interpret(ast, &mut ram);
        assert_eq!(result, expected)
    }

    #[test]
    fn test_interpreter_float() {
        let mut ram: HashMap<String, Parameters> = HashMap::new();
        let expected = Parameters::Float(2.0);
        let ast = Ast::Node {
            value: Parameters::Float(2.0),
            left: Box::from(Ast::Nil),
            right: Box::from(Ast::Nil),
        };
        let result = interpret(ast, &mut ram);
        assert_eq!(result, expected)
    }

    #[test]
    fn test_interpreter_plus_operation() {
        let mut ram: HashMap<String, Parameters> = HashMap::new();
        let expected = Parameters::Int(2);
        let ast = Ast::Node {
            value: Parameters::PlusOperation,
            left: Box::from(Ast::new(Parameters::Int(1))),
            right: Box::from(Ast::new(Parameters::Int(1))),
        };
        let result = interpret(ast, &mut ram);
        assert_eq!(result, expected)
    }

    #[test]
    fn test_interpreter_minus_operation() {
        let mut ram: HashMap<String, Parameters> = HashMap::new();
        let expected = Parameters::Int(0);
        let ast = Ast::Node {
            value: Parameters::MinusOperation,
            left: Box::from(Ast::new(Parameters::Int(1))),
            right: Box::from(Ast::new(Parameters::Int(1))),
        };
        let result = interpret(ast, &mut ram);
        assert_eq!(result, expected)
    }

    #[test]
    fn test_interpreter_mult_operation() {
        let mut ram: HashMap<String, Parameters> = HashMap::new();
        let expected = Parameters::Int(1);
        let ast = Ast::Node {
            value: Parameters::MultiplicationOperation,
            left: Box::from(Ast::new(Parameters::Int(1))),
            right: Box::from(Ast::new(Parameters::Int(1))),
        };
        let result = interpret(ast, &mut ram);
        assert_eq!(result, expected)
    }

    #[test]
    fn test_interpreter_divide_operation() {
        let mut ram: HashMap<String, Parameters> = HashMap::new();
        let expected = Parameters::Float(1.0);
        let ast = Ast::Node {
            value: Parameters::DivideOperation,
            left: Box::from(Ast::new(Parameters::Int(1))),
            right: Box::from(Ast::new(Parameters::Int(1))),
        };
        let result = interpret(ast, &mut ram);
        assert_eq!(result, expected)
    }
}
