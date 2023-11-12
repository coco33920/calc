use std::collections::HashMap;

use crate::parsing::ast::Parameters;

pub fn add(i: Parameters, i2: Parameters) -> Parameters {
    match (i, i2) {
        (Parameters::Null, Parameters::Int(v)) => Parameters::Int(v),
        (Parameters::Null, Parameters::Float(f)) => Parameters::Float(f),
        (Parameters::Int(v), Parameters::Null) => Parameters::Int(v),
        (Parameters::Float(f), Parameters::Null) => Parameters::Float(f),
        (Parameters::Int(v), Parameters::Int(v2)) => Parameters::Int(v + v2),
        (Parameters::Int(v), Parameters::Float(f)) => Parameters::Float((v as f64) + f),
        (Parameters::Float(v), Parameters::Float(f)) => Parameters::Float(v + f),
        (Parameters::Float(v), Parameters::Int(i1)) => Parameters::Float(v + (i1 as f64)),
        _ => Parameters::Null,
    }
}

pub fn minus(i: Parameters, i2: Parameters) -> Parameters {
    match (i, i2) {
        (Parameters::Null, Parameters::Int(v)) => Parameters::Int(-v),
        (Parameters::Null, Parameters::Float(f)) => Parameters::Float(-f),
        (Parameters::Int(v), Parameters::Null) => Parameters::Int(-v),
        (Parameters::Float(f), Parameters::Null) => Parameters::Float(-f),
        (Parameters::Int(v), Parameters::Int(v2)) => Parameters::Int(v - v2),
        (Parameters::Int(v), Parameters::Float(f)) => Parameters::Float((v as f64) - f),
        (Parameters::Float(v), Parameters::Float(f)) => Parameters::Float(v - f),
        (Parameters::Float(v), Parameters::Int(i1)) => Parameters::Float(v - (i1 as f64)),
        _ => Parameters::Null,
    }
}

pub fn mult(i: Parameters, i2: Parameters) -> Parameters {
    match (i, i2) {
        (Parameters::Null, Parameters::Int(v)) => Parameters::Int(v),
        (Parameters::Null, Parameters::Float(f)) => Parameters::Float(f),
        (Parameters::Int(v), Parameters::Null) => Parameters::Int(v),
        (Parameters::Float(f), Parameters::Null) => Parameters::Float(f),
        (Parameters::Int(v), Parameters::Int(v2)) => Parameters::Int(v * v2),
        (Parameters::Int(v), Parameters::Float(f)) => Parameters::Float((v as f64) * f),
        (Parameters::Float(v), Parameters::Float(f)) => Parameters::Float(v * f),
        (Parameters::Float(v), Parameters::Int(i1)) => Parameters::Float(v * (i1 as f64)),
        _ => Parameters::Null,
    }
}

pub fn divide(i: Parameters, i2: Parameters) -> Parameters {
    match (i, i2) {
        (Parameters::Null, Parameters::Int(v)) => Parameters::Int(v),
        (Parameters::Null, Parameters::Float(f)) => Parameters::Float(f),
        (Parameters::Int(v), Parameters::Null) => Parameters::Int(v),
        (Parameters::Float(f), Parameters::Null) => Parameters::Float(f),
        (Parameters::Int(v), Parameters::Int(v2)) => Parameters::Float((v as f64) / (v2 as f64)),
        (Parameters::Int(v), Parameters::Float(f)) => Parameters::Float((v as f64) / f),
        (Parameters::Float(v), Parameters::Float(f)) => Parameters::Float(v / f),
        (Parameters::Float(v), Parameters::Int(i1)) => Parameters::Float(v / (i1 as f64)),
        _ => Parameters::Null,
    }
}

pub fn assign(ram: &HashMap<String, Parameters>, s: Parameters, s2: Parameters) -> () {
    match s {
        Parameters::Identifier(s) => {
            (ram.clone()).insert(s, s2);
            ()
        }
        _ => (),
    }
}

#[cfg(test)]
mod test {
    use crate::interpreting::function::{add, divide, minus, mult};
    use crate::parsing::ast::Parameters;

    #[test]
    pub fn test_add_null() {
        let expected = Parameters::Int(1);
        let result = add(Parameters::Int(1), Parameters::Null);
        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_add_simple() {
        let expected = Parameters::Int(2);
        let result = add(Parameters::Int(1), Parameters::Int(1));
        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_add_float() {
        let expected = Parameters::Float(2.1);
        let result = add(Parameters::Float(0.1), Parameters::Float(2.0));
        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_add_int_float() {
        let expected = Parameters::Float(2.1);
        let result = add(Parameters::Int(2), Parameters::Float(0.1));
        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_add_float_int() {
        let expected = Parameters::Float(2.1);
        let result = add(Parameters::Float(0.1), Parameters::Int(2));
        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_minus_null() {
        let expected = Parameters::Int(-1);
        let result = minus(Parameters::Int(1), Parameters::Null);
        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_minus_null_rev() {
        let expected = Parameters::Int(-1);
        let result = minus(Parameters::Null, Parameters::Int(1));
        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_minus_simple() {
        let expected = Parameters::Int(0);
        let result = minus(Parameters::Int(1), Parameters::Int(1));
        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_minus_float() {
        let expected = Parameters::Float(1.9);
        let result = minus(Parameters::Float(2.0), Parameters::Float(0.1));
        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_minus_int_float() {
        let expected = Parameters::Float(1.9);
        let result = minus(Parameters::Int(2), Parameters::Float(0.1));
        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_minus_float_int() {
        let expected = Parameters::Float(-1.9);
        let result = minus(Parameters::Float(0.1), Parameters::Int(2));
        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_mult_null() {
        let expected = Parameters::Int(1);
        let result = mult(Parameters::Int(1), Parameters::Null);
        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_mult_simple() {
        let expected = Parameters::Int(2);
        let result = mult(Parameters::Int(1), Parameters::Int(2));
        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_mult_float() {
        let expected = Parameters::Float(0.2);
        let result = mult(Parameters::Float(0.1), Parameters::Float(2.0));
        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_mult_int_float() {
        let expected = Parameters::Float(0.2);
        let result = mult(Parameters::Int(2), Parameters::Float(0.1));
        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_mult_float_int() {
        let expected = Parameters::Float(0.2);
        let result = mult(Parameters::Float(0.1), Parameters::Int(2));
        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_divide_null() {
        let expected = Parameters::Int(1);
        let result = divide(Parameters::Int(1), Parameters::Null);
        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_divide_simple() {
        let expected = Parameters::Float(1.0);
        let result = divide(Parameters::Int(1), Parameters::Int(1));
        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_divide_float() {
        let expected = Parameters::Float(0.05);
        let result = divide(Parameters::Float(0.1), Parameters::Float(2.0));
        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_divide_int_float() {
        let expected = Parameters::Float(20.0);
        let result = divide(Parameters::Int(2), Parameters::Float(0.1));
        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_divide_float_int() {
        let expected = Parameters::Float(0.05);
        let result = divide(Parameters::Float(0.1), Parameters::Int(2));
        assert_eq!(result, expected);
    }
}
