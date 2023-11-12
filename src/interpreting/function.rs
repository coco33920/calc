use std::collections::HashMap;

use crate::parsing::ast::Parameters;

pub fn apply_operator(
    value: Parameters,
    value2: Parameters,
    ram: Option<&HashMap<String, Parameters>>,
    f: fn(Parameters, Parameters, Option<&HashMap<String, Parameters>>) -> Parameters,
) -> Parameters {
    let s = match value {
        Parameters::Identifier(s) => s,
        _ => "".to_string(),
    };
    if s == "".to_string() {
        return Parameters::Null;
    }
    match ram {
        None => value2,
        Some(i_ram) => {
            let value = i_ram.get(&s);
            match value {
                None => value2,
                Some(val) => f(val.clone(), value2.clone(), ram),
            }
        }
    }
}

pub fn add(i: Parameters, i2: Parameters, ram: Option<&HashMap<String, Parameters>>) -> Parameters {
    match (i, i2) {
        (Parameters::Null, Parameters::Int(v)) => Parameters::Int(v),
        (Parameters::Null, Parameters::Float(f)) => Parameters::Float(f),
        (Parameters::Int(v), Parameters::Null) => Parameters::Int(v),
        (Parameters::Float(f), Parameters::Null) => Parameters::Float(f),
        (Parameters::Int(v), Parameters::Int(v2)) => Parameters::Int(v + v2),
        (Parameters::Int(v), Parameters::Float(f)) => Parameters::Float((v as f64) + f),
        (Parameters::Float(v), Parameters::Float(f)) => Parameters::Float(v + f),
        (Parameters::Float(v), Parameters::Int(i1)) => Parameters::Float(v + (i1 as f64)),
        (Parameters::Identifier(s), Parameters::Identifier(s2)) => apply_operator(
            Parameters::Identifier(s),
            Parameters::Identifier(s2),
            ram,
            add,
        ),
        (Parameters::Identifier(s), Parameters::Int(i)) => {
            apply_operator(Parameters::Identifier(s), Parameters::Int(i), ram, add)
        }
        (Parameters::Int(i), Parameters::Identifier(s)) => {
            apply_operator(Parameters::Identifier(s), Parameters::Int(i), ram, add)
        }
        (Parameters::Identifier(s), Parameters::Float(i)) => {
            apply_operator(Parameters::Identifier(s), Parameters::Float(i), ram, add)
        }
        (Parameters::Float(i), Parameters::Identifier(s)) => {
            apply_operator(Parameters::Identifier(s), Parameters::Float(i), ram, add)
        }
        _ => Parameters::Null,
    }
}

pub fn minus(
    i: Parameters,
    i2: Parameters,
    ram: Option<&HashMap<String, Parameters>>,
) -> Parameters {
    match (i, i2) {
        (Parameters::Null, Parameters::Int(v)) => Parameters::Int(-v),
        (Parameters::Null, Parameters::Float(f)) => Parameters::Float(-f),
        (Parameters::Int(v), Parameters::Null) => Parameters::Int(-v),
        (Parameters::Float(f), Parameters::Null) => Parameters::Float(-f),
        (Parameters::Int(v), Parameters::Int(v2)) => Parameters::Int(v - v2),
        (Parameters::Int(v), Parameters::Float(f)) => Parameters::Float((v as f64) - f),
        (Parameters::Float(v), Parameters::Float(f)) => Parameters::Float(v - f),
        (Parameters::Float(v), Parameters::Int(i1)) => Parameters::Float(v - (i1 as f64)),
        (Parameters::Identifier(s), Parameters::Identifier(s2)) => apply_operator(
            Parameters::Identifier(s),
            Parameters::Identifier(s2),
            ram,
            minus,
        ),
        (Parameters::Identifier(s), Parameters::Int(i)) => {
            apply_operator(Parameters::Identifier(s), Parameters::Int(i), ram, minus)
        }
        (Parameters::Int(i), Parameters::Identifier(s)) => {
            let v = apply_operator(Parameters::Identifier(s), Parameters::Int(i), ram, minus);
            match v {
                Parameters::Int(i) => Parameters::Int(-i),
                _ => Parameters::Null,
            }
        }
        (Parameters::Identifier(s), Parameters::Float(i)) => {
            apply_operator(Parameters::Identifier(s), Parameters::Float(i), ram, minus)
        }
        (Parameters::Float(i), Parameters::Identifier(s)) => {
            let v = apply_operator(Parameters::Identifier(s), Parameters::Float(i), ram, minus);
            match v {
                Parameters::Float(i) => Parameters::Float(-i),
                _ => Parameters::Null,
            }
        }
        _ => Parameters::Null,
    }
}

pub fn mult(
    i: Parameters,
    i2: Parameters,
    ram: Option<&HashMap<String, Parameters>>,
) -> Parameters {
    match (i, i2) {
        (Parameters::Null, Parameters::Int(v)) => Parameters::Int(v),
        (Parameters::Null, Parameters::Float(f)) => Parameters::Float(f),
        (Parameters::Int(v), Parameters::Null) => Parameters::Int(v),
        (Parameters::Float(f), Parameters::Null) => Parameters::Float(f),
        (Parameters::Int(v), Parameters::Int(v2)) => Parameters::Int(v * v2),
        (Parameters::Int(v), Parameters::Float(f)) => Parameters::Float((v as f64) * f),
        (Parameters::Float(v), Parameters::Float(f)) => Parameters::Float(v * f),
        (Parameters::Float(v), Parameters::Int(i1)) => Parameters::Float(v * (i1 as f64)),
        (Parameters::Identifier(s), Parameters::Identifier(s2)) => apply_operator(
            Parameters::Identifier(s),
            Parameters::Identifier(s2),
            ram,
            mult,
        ),
        (Parameters::Identifier(s), Parameters::Int(i)) => {
            apply_operator(Parameters::Identifier(s), Parameters::Int(i), ram, mult)
        }
        (Parameters::Int(i), Parameters::Identifier(s)) => {
            apply_operator(Parameters::Identifier(s), Parameters::Int(i), ram, mult)
        }
        (Parameters::Identifier(s), Parameters::Float(i)) => {
            apply_operator(Parameters::Identifier(s), Parameters::Float(i), ram, mult)
        }
        (Parameters::Float(i), Parameters::Identifier(s)) => {
            apply_operator(Parameters::Identifier(s), Parameters::Float(i), ram, mult)
        }
        _ => Parameters::Null,
    }
}

pub fn divide(
    i: Parameters,
    i2: Parameters,
    ram: Option<&HashMap<String, Parameters>>,
) -> Parameters {
    match (i, i2) {
        (Parameters::Null, Parameters::Int(v)) => Parameters::Int(v),
        (Parameters::Null, Parameters::Float(f)) => Parameters::Float(f),
        (Parameters::Int(v), Parameters::Null) => Parameters::Int(v),
        (Parameters::Float(f), Parameters::Null) => Parameters::Float(f),
        (Parameters::Int(v), Parameters::Int(v2)) => Parameters::Float((v as f64) / (v2 as f64)),
        (Parameters::Int(v), Parameters::Float(f)) => Parameters::Float((v as f64) / f),
        (Parameters::Float(v), Parameters::Float(f)) => Parameters::Float(v / f),
        (Parameters::Float(v), Parameters::Int(i1)) => Parameters::Float(v / (i1 as f64)),
        (Parameters::Identifier(s), Parameters::Identifier(s2)) => apply_operator(
            Parameters::Identifier(s),
            Parameters::Identifier(s2),
            ram,
            divide,
        ),
        (Parameters::Identifier(s), Parameters::Int(i)) => {
            apply_operator(Parameters::Identifier(s), Parameters::Int(i), ram, divide)
        }
        (Parameters::Int(i), Parameters::Identifier(s)) => {
            let v = apply_operator(Parameters::Identifier(s), Parameters::Int(i), ram, divide);
            match v {
                Parameters::Float(i) => Parameters::Float(1.0 / i),
                _ => Parameters::Null,
            }
        }
        (Parameters::Identifier(s), Parameters::Float(i)) => {
            apply_operator(Parameters::Identifier(s), Parameters::Float(i), ram, divide)
        }
        (Parameters::Float(i), Parameters::Identifier(s)) => {
            let v = apply_operator(Parameters::Identifier(s), Parameters::Float(i), ram, divide);
            match v {
                Parameters::Float(i) => Parameters::Float(1.0 / i),
                _ => Parameters::Null,
            }
        }
        _ => Parameters::Null,
    }
}

pub fn assign(s: Parameters, s2: Parameters) -> (String, Parameters) {
    match s {
        Parameters::Identifier(s) => (s, s2),
        _ => ("".to_string(), s2),
    }
}

#[cfg(test)]
mod test {
    use crate::interpreting::function::{add, divide, minus, mult};
    use crate::parsing::ast::Parameters;

    #[test]
    pub fn test_add_null() {
        let expected = Parameters::Int(1);
        let result = add(Parameters::Int(1), Parameters::Null, None);
        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_add_simple() {
        let expected = Parameters::Int(2);
        let result = add(Parameters::Int(1), Parameters::Int(1), None);
        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_add_float() {
        let expected = Parameters::Float(2.1);
        let result = add(Parameters::Float(0.1), Parameters::Float(2.0), None);
        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_add_int_float() {
        let expected = Parameters::Float(2.1);
        let result = add(Parameters::Int(2), Parameters::Float(0.1), None);
        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_add_float_int() {
        let expected = Parameters::Float(2.1);
        let result = add(Parameters::Float(0.1), Parameters::Int(2), None);
        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_minus_null() {
        let expected = Parameters::Int(-1);
        let result = minus(Parameters::Int(1), Parameters::Null, None);
        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_minus_null_rev() {
        let expected = Parameters::Int(-1);
        let result = minus(Parameters::Null, Parameters::Int(1), None);
        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_minus_simple() {
        let expected = Parameters::Int(0);
        let result = minus(Parameters::Int(1), Parameters::Int(1), None);
        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_minus_float() {
        let expected = Parameters::Float(1.9);
        let result = minus(Parameters::Float(2.0), Parameters::Float(0.1), None);
        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_minus_int_float() {
        let expected = Parameters::Float(1.9);
        let result = minus(Parameters::Int(2), Parameters::Float(0.1), None);
        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_minus_float_int() {
        let expected = Parameters::Float(-1.9);
        let result = minus(Parameters::Float(0.1), Parameters::Int(2), None);
        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_mult_null() {
        let expected = Parameters::Int(1);
        let result = mult(Parameters::Int(1), Parameters::Null, None);
        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_mult_simple() {
        let expected = Parameters::Int(2);
        let result = mult(Parameters::Int(1), Parameters::Int(2), None);
        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_mult_float() {
        let expected = Parameters::Float(0.2);
        let result = mult(Parameters::Float(0.1), Parameters::Float(2.0), None);
        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_mult_int_float() {
        let expected = Parameters::Float(0.2);
        let result = mult(Parameters::Int(2), Parameters::Float(0.1), None);
        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_mult_float_int() {
        let expected = Parameters::Float(0.2);
        let result = mult(Parameters::Float(0.1), Parameters::Int(2), None);
        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_divide_null() {
        let expected = Parameters::Int(1);
        let result = divide(Parameters::Int(1), Parameters::Null, None);
        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_divide_simple() {
        let expected = Parameters::Float(1.0);
        let result = divide(Parameters::Int(1), Parameters::Int(1), None);
        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_divide_float() {
        let expected = Parameters::Float(0.05);
        let result = divide(Parameters::Float(0.1), Parameters::Float(2.0), None);
        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_divide_int_float() {
        let expected = Parameters::Float(20.0);
        let result = divide(Parameters::Int(2), Parameters::Float(0.1), None);
        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_divide_float_int() {
        let expected = Parameters::Float(0.05);
        let result = divide(Parameters::Float(0.1), Parameters::Int(2), None);
        assert_eq!(result, expected);
    }
}
