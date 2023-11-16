use std::collections::HashMap;

use crate::interpreting::function::{
    add, and, assign, divide, equal, expo, greater, greater_or_equal, lesser, lesser_or_equal,
    minus, mult, not, or,
};
use crate::interpreting::stdlib::exec;
use crate::parsing::ast::{Ast, Parameters};

pub fn interpret(
    ast: &Ast,
    mut ram: &mut HashMap<String, Parameters>,
    mut function: &mut HashMap<String, (Vec<Ast>, Ast)>,
) -> Parameters {
    match ast {
        Ast::Nil => Parameters::Null,
        Ast::Node {
            value: v,
            left: l,
            right: r,
        } => {
            let param1 = interpret(l, &mut ram, &mut function);
            let param2 = interpret(r, &mut ram, &mut function);
            let last = match v {
                Parameters::PlusOperation => add(param1, param2, Some(&ram)),
                Parameters::MinusOperation => minus(param1, param2, Some(&ram)),
                Parameters::MultiplicationOperation => mult(param1, param2, Some(&ram)),
                Parameters::DivideOperation => divide(param1, param2, Some(&ram)),
                Parameters::ExpoOperation => expo(param1, param2, Some(&ram)),
                Parameters::Equal => equal(param1, param2, Some(&ram)),
                Parameters::Not => not(param1, param2, Some(&ram)),
                Parameters::GreaterOperation => greater(param1, param2, Some(&ram)),
                Parameters::GreaterOrEqualOperation => greater_or_equal(param1, param2, Some(&ram)),
                Parameters::LesserOperation => lesser(param1, param2, Some(&ram)),
                Parameters::LesserOrEqualOperation => lesser_or_equal(param1, param2, Some(&ram)),
                Parameters::AndOperation => and(param1, param2, Some(&ram)),
                Parameters::OrOperation => or(param1, param2, Some(&ram)),
                Parameters::Assign => match *(l.clone()) {
                    Ast::Call { name: n, lst: list } => {
                        if n.as_str() != "" {
                            (function).insert(n.to_string(), (list, *r.clone()));
                        }
                        Parameters::Null
                    }
                    _ => {
                        let (a, b) = assign(param1.clone(), param2.clone());
                        if a != "".to_string() {
                            (ram).insert(a, b);
                        }
                        Parameters::Null
                    }
                },
                Parameters::Float(f) => Parameters::Float(*f),
                Parameters::Int(i) => Parameters::Int(*i),
                Parameters::Identifier(s) => Parameters::Identifier(s.clone()),
                Parameters::Bool(b) => Parameters::Bool(*b),
                Parameters::Null => Parameters::Null,
                Parameters::Vector(a) => {
                    let mut vec = Vec::new();
                    (*a).clone()
                        .into_iter()
                        .map(|a| interpret(&a, ram, function))
                        .for_each(|s| vec.push(s));
                    Parameters::InterpreterVector(Box::from(vec))
                }
                Parameters::InterpreterVector(a) => Parameters::InterpreterVector(a.clone()),
            };
            last.clone()
        }
        Ast::Call { name: n, lst: list } => {
            let v: Vec<Parameters> = list.iter().map(|x| interpret(x, ram, function)).collect();
            exec(n.to_string(), v, Some(&mut ram), Some(&mut function))
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
        let mut function: HashMap<String, (Vec<Ast>, Ast)> = HashMap::new();
        let expected = Parameters::Int(2);
        let ast = Ast::Node {
            value: Parameters::Int(2),
            left: Box::from(Ast::Nil),
            right: Box::from(Ast::Nil),
        };
        let result = interpret(&ast, &mut ram, &mut function);
        assert_eq!(result, expected)
    }

    #[test]
    fn test_interpreter_float() {
        let mut ram: HashMap<String, Parameters> = HashMap::new();
        let mut function: HashMap<String, (Vec<Ast>, Ast)> = HashMap::new();
        let expected = Parameters::Float(2.0);
        let ast = Ast::Node {
            value: Parameters::Float(2.0),
            left: Box::from(Ast::Nil),
            right: Box::from(Ast::Nil),
        };
        let result = interpret(&ast, &mut ram, &mut function);
        assert_eq!(result, expected)
    }

    #[test]
    fn test_interpreter_plus_operation() {
        let mut ram: HashMap<String, Parameters> = HashMap::new();
        let mut function: HashMap<String, (Vec<Ast>, Ast)> = HashMap::new();
        let expected = Parameters::Int(2);
        let ast = Ast::Node {
            value: Parameters::PlusOperation,
            left: Box::from(Ast::new(Parameters::Int(1))),
            right: Box::from(Ast::new(Parameters::Int(1))),
        };
        let result = interpret(&ast, &mut ram, &mut function);
        assert_eq!(result, expected)
    }

    #[test]
    fn test_interpreter_minus_operation() {
        let mut ram: HashMap<String, Parameters> = HashMap::new();
        let mut function: HashMap<String, (Vec<Ast>, Ast)> = HashMap::new();
        let expected = Parameters::Int(0);
        let ast = Ast::Node {
            value: Parameters::MinusOperation,
            left: Box::from(Ast::new(Parameters::Int(1))),
            right: Box::from(Ast::new(Parameters::Int(1))),
        };
        let result = interpret(&ast, &mut ram, &mut function);
        assert_eq!(result, expected)
    }

    #[test]
    fn test_interpreter_mult_operation() {
        let mut ram: HashMap<String, Parameters> = HashMap::new();
        let mut function: HashMap<String, (Vec<Ast>, Ast)> = HashMap::new();
        let expected = Parameters::Int(1);
        let ast = Ast::Node {
            value: Parameters::MultiplicationOperation,
            left: Box::from(Ast::new(Parameters::Int(1))),
            right: Box::from(Ast::new(Parameters::Int(1))),
        };
        let result = interpret(&ast, &mut ram, &mut function);
        assert_eq!(result, expected)
    }

    #[test]
    fn test_interpreter_divide_operation() {
        let mut ram: HashMap<String, Parameters> = HashMap::new();
        let mut function: HashMap<String, (Vec<Ast>, Ast)> = HashMap::new();
        let expected = Parameters::Float(1.0);
        let ast = Ast::Node {
            value: Parameters::DivideOperation,
            left: Box::from(Ast::new(Parameters::Int(1))),
            right: Box::from(Ast::new(Parameters::Int(1))),
        };
        let result = interpret(&ast, &mut ram, &mut function);
        assert_eq!(result, expected)
    }
}
