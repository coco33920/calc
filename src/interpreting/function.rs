use std::collections::HashMap;

use crate::parsing::ast::Parameters;
use crate::parsing::ast::Parameters::Bool;

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

pub fn apply_operator_reverse(
    value: Parameters,
    value2: Parameters,
    ram: Option<&HashMap<String, Parameters>>,
    f: fn(Parameters, Parameters, Option<&HashMap<String, Parameters>>) -> Parameters,
) -> Parameters {
    let s = match value2 {
        Parameters::Identifier(s) => s,
        _ => "".to_string(),
    };
    if s == "".to_string() {
        return Parameters::Null;
    }
    match ram {
        None => value,
        Some(i_ram) => {
            let val3 = i_ram.get(&s);
            match val3 {
                None => value,
                Some(val) => f(value.clone(), val.clone(), ram),
            }
        }
    }
}

pub fn add(i: Parameters, i2: Parameters, ram: Option<&HashMap<String, Parameters>>) -> Parameters {
    match (i, i2) {
        (Parameters::Null, Parameters::Int(v)) => Parameters::Int(v),
        (Parameters::Null, Parameters::Float(f)) => Parameters::Float(f),
        (Parameters::Null, Parameters::InterpreterVector(vec)) => {
            Parameters::InterpreterVector(vec.clone())
        }
        (Parameters::InterpreterVector(vec), Parameters::Null) => {
            Parameters::InterpreterVector(vec.clone())
        }
        (Parameters::Int(v), Parameters::Null) => Parameters::Int(v),
        (Parameters::Float(f), Parameters::Null) => Parameters::Float(f),
        (Parameters::Int(v), Parameters::Int(v2)) => Parameters::Int(v + v2),
        (Parameters::Int(v), Parameters::Float(f)) => Parameters::Float((v as f64) + f),
        (Parameters::Float(v), Parameters::Float(f)) => Parameters::Float(v + f),
        (Parameters::Float(v), Parameters::Int(i1)) => Parameters::Float(v + (i1 as f64)),
        (Parameters::InterpreterVector(vec), Parameters::InterpreterVector(vec2)) => {
            let mut res = Vec::new();
            vec.into_iter()
                .zip(vec2.into_iter())
                .map(|(x, y)| add(x.clone(), y.clone(), ram))
                .for_each(|s| res.push(s));
            Parameters::InterpreterVector(res.as_slice().into())
        }
        (Bool(_), Parameters::Int(i)) => Parameters::Int(i),
        (Bool(_), Parameters::Float(i)) => Parameters::Float(i),
        (Parameters::Int(i), Bool(_)) => Parameters::Int(i),
        (Parameters::Float(i), Bool(_)) => Parameters::Float(i),
        (Bool(b), Parameters::Null) => Bool(b),
        (Parameters::Null, Bool(b)) => Bool(b),
        (Bool(b), Bool(b2)) => Bool(b && b2),
        (Parameters::Identifier(s), Parameters::Identifier(s2)) => apply_operator(
            Parameters::Identifier(s),
            Parameters::Identifier(s2),
            ram,
            add,
        ),
        (Parameters::Identifier(s), Parameters::Int(i)) => {
            apply_operator(Parameters::Identifier(s), Parameters::Int(i), ram, add)
        }
        (Parameters::Null, Parameters::Identifier(s)) => {
            apply_operator(Parameters::Identifier(s), Parameters::Null, ram, add)
        }
        (Parameters::Identifier(s), Parameters::Null) => {
            apply_operator(Parameters::Identifier(s), Parameters::Null, ram, add)
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
        (Parameters::Identifier(s), Parameters::InterpreterVector(vec)) => apply_operator(
            Parameters::Identifier(s),
            Parameters::InterpreterVector(vec.clone()),
            ram,
            add,
        ),
        (Parameters::InterpreterVector(vec), Parameters::Identifier(s)) => apply_operator(
            Parameters::Identifier(s),
            Parameters::InterpreterVector(vec.clone()),
            ram,
            add,
        ),
        (Bool(b), Parameters::Identifier(s)) => {
            apply_operator_reverse(Bool(b), Parameters::Identifier(s), ram, add)
        }
        (Parameters::Identifier(s), Bool(b)) => {
            apply_operator(Parameters::Identifier(s), Bool(b), ram, add)
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

        (Parameters::InterpreterVector(vec), Parameters::Null) => {
            let mut res = Vec::new();
            vec.into_iter()
                .map(|x| minus(Parameters::Null, x.clone(), ram))
                .for_each(|z| res.push(z));
            Parameters::InterpreterVector(res.as_slice().into())
        }

        (Parameters::Null, Parameters::InterpreterVector(vec)) => {
            let mut res = Vec::new();
            vec.into_iter()
                .map(|x| minus(Parameters::Null, x.clone(), ram))
                .for_each(|z| res.push(z));
            Parameters::InterpreterVector(res.as_slice().into())
        }

        (Parameters::InterpreterVector(vec), Parameters::InterpreterVector(vec2)) => {
            let mut res = Vec::new();
            vec.into_iter()
                .zip(vec2.into_iter())
                .map(|(x, y)| minus(x.clone(), y.clone(), ram))
                .for_each(|z| res.push(z));
            Parameters::InterpreterVector(res.as_slice().into())
        }
        (Parameters::Int(v), Parameters::Float(f)) => Parameters::Float((v as f64) - f),
        (Parameters::Float(v), Parameters::Float(f)) => Parameters::Float(v - f),
        (Parameters::Float(v), Parameters::Int(i1)) => Parameters::Float(v - (i1 as f64)),

        (Bool(_), Parameters::Int(i)) => Parameters::Int(i),
        (Bool(_), Parameters::Float(i)) => Parameters::Float(i),
        (Parameters::Int(i), Bool(_)) => Parameters::Int(i),
        (Parameters::Float(i), Bool(_)) => Parameters::Float(i),
        (Bool(b), Parameters::Null) => Bool(b),
        (Parameters::Null, Bool(b)) => Bool(b),
        (Bool(b), Bool(b2)) => Bool(b && b2),
        (Parameters::Identifier(s), Parameters::Identifier(s2)) => apply_operator(
            Parameters::Identifier(s),
            Parameters::Identifier(s2),
            ram,
            minus,
        ),
        (Parameters::Identifier(s), Parameters::Int(i)) => {
            apply_operator(Parameters::Identifier(s), Parameters::Int(i), ram, minus)
        }
        (Parameters::Null, Parameters::Identifier(s)) => {
            apply_operator(Parameters::Identifier(s), Parameters::Null, ram, minus)
        }
        (Parameters::Identifier(s), Parameters::Null) => {
            apply_operator(Parameters::Identifier(s), Parameters::Null, ram, minus)
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

        (Parameters::InterpreterVector(vec), Parameters::Identifier(s)) => apply_operator_reverse(
            Parameters::InterpreterVector(vec.clone()),
            Parameters::Identifier(s),
            ram,
            minus,
        ),
        (Parameters::Identifier(s), Parameters::InterpreterVector(vec)) => apply_operator(
            Parameters::Identifier(s),
            Parameters::InterpreterVector(vec.clone()),
            ram,
            minus,
        ),
        (Bool(b), Parameters::Identifier(s)) => {
            apply_operator_reverse(Bool(b), Parameters::Identifier(s), ram, minus)
        }
        (Parameters::Identifier(s), Bool(b)) => {
            apply_operator(Parameters::Identifier(s), Bool(b), ram, minus)
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

        (Parameters::Null, Parameters::InterpreterVector(vec)) => {
            Parameters::InterpreterVector(vec.clone())
        }
        (Parameters::InterpreterVector(vec), Parameters::Null) => {
            Parameters::InterpreterVector(vec.clone())
        }
        (Parameters::InterpreterVector(vec), Parameters::Int(v)) => {
            let mut result = Vec::new();
            vec.into_iter()
                .map(|x| mult(x.clone(), Parameters::Int(v), ram))
                .for_each(|x| result.push(x));
            Parameters::InterpreterVector(result.as_slice().into())
        }
        (Parameters::Int(v), Parameters::InterpreterVector(vec)) => {
            let mut result = Vec::new();
            vec.into_iter()
                .map(|x| mult(x.clone(), Parameters::Int(v), ram))
                .for_each(|x| result.push(x));
            Parameters::InterpreterVector(result.as_slice().into())
        }
        (Parameters::InterpreterVector(vec), Parameters::Float(v)) => {
            let mut result = Vec::new();
            vec.into_iter()
                .map(|x| mult(x.clone(), Parameters::Float(v), ram))
                .for_each(|x| result.push(x));
            Parameters::InterpreterVector(result.as_slice().into())
        }
        (Parameters::Float(v), Parameters::InterpreterVector(vec)) => {
            let mut result = Vec::new();
            vec.into_iter()
                .map(|x| mult(x.clone(), Parameters::Float(v), ram))
                .for_each(|x| result.push(x));
            Parameters::InterpreterVector(result.as_slice().into())
        }

        (Parameters::InterpreterVector(vec), Parameters::InterpreterVector(vec2)) => {
            let mut sum = Parameters::Null;
            (*vec)
                .into_iter()
                .zip(vec2.into_iter())
                .map(|(a, b)| mult(a.clone(), b.clone(), ram))
                .for_each(|x| sum = add(sum.clone(), x, ram));

            match sum {
                Parameters::Int(i) => Parameters::Int(i),
                Parameters::Float(f) => Parameters::Float(f),
                _ => Parameters::Float(f64::NAN),
            }
        }

        (Bool(_), Parameters::Int(i)) => Parameters::Int(i),
        (Bool(_), Parameters::Float(i)) => Parameters::Float(i),
        (Parameters::Int(i), Bool(_)) => Parameters::Int(i),
        (Parameters::Float(i), Bool(_)) => Parameters::Float(i),
        (Bool(b), Parameters::Null) => Bool(b),
        (Parameters::Null, Bool(b)) => Bool(b),
        (Bool(b), Bool(b2)) => Bool(b && b2),
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
        (Parameters::Identifier(s), Parameters::InterpreterVector(vec)) => apply_operator(
            Parameters::Identifier(s),
            Parameters::InterpreterVector(vec.clone()),
            ram,
            mult,
        ),
        (Parameters::InterpreterVector(vec), Parameters::Identifier(s)) => apply_operator_reverse(
            Parameters::InterpreterVector(vec.clone()),
            Parameters::Identifier(s),
            ram,
            mult,
        ),
        (Parameters::Null, Parameters::Identifier(s)) => {
            apply_operator(Parameters::Identifier(s), Parameters::Null, ram, mult)
        }
        (Parameters::Identifier(s), Parameters::Null) => {
            apply_operator(Parameters::Identifier(s), Parameters::Null, ram, mult)
        }
        (Parameters::Identifier(s), Parameters::Float(i)) => {
            apply_operator(Parameters::Identifier(s), Parameters::Float(i), ram, mult)
        }
        (Parameters::Float(i), Parameters::Identifier(s)) => {
            apply_operator(Parameters::Identifier(s), Parameters::Float(i), ram, mult)
        }
        (Bool(b), Parameters::Identifier(s)) => {
            apply_operator_reverse(Bool(b), Parameters::Identifier(s), ram, mult)
        }
        (Parameters::Identifier(s), Bool(b)) => {
            apply_operator(Parameters::Identifier(s), Bool(b), ram, mult)
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
        (Parameters::Null, Parameters::InterpreterVector(vec)) => {
            Parameters::InterpreterVector(vec.clone())
        }
        (Parameters::InterpreterVector(vec), Parameters::Null) => {
            Parameters::InterpreterVector(vec.clone())
        }
        (Bool(_), Parameters::Int(i)) => Parameters::Int(i),
        (Bool(_), Parameters::Float(i)) => Parameters::Float(i),
        (Parameters::Int(i), Bool(_)) => Parameters::Int(i),
        (Parameters::Float(i), Bool(_)) => Parameters::Float(i),
        (Bool(b), Parameters::Null) => Bool(b),
        (Parameters::Null, Bool(b)) => Bool(b),
        (Bool(b), Bool(b2)) => Bool(b && b2),
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
        (Parameters::Null, Parameters::Identifier(s)) => {
            apply_operator(Parameters::Identifier(s), Parameters::Null, ram, divide)
        }
        (Parameters::Identifier(s), Parameters::Null) => {
            apply_operator(Parameters::Identifier(s), Parameters::Null, ram, divide)
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
        (Bool(b), Parameters::Identifier(s)) => {
            apply_operator_reverse(Bool(b), Parameters::Identifier(s), ram, divide)
        }
        (Parameters::Identifier(s), Bool(b)) => {
            apply_operator(Parameters::Identifier(s), Bool(b), ram, divide)
        }
        _ => Parameters::Null,
    }
}

pub fn expo(
    i: Parameters,
    i2: Parameters,
    ram: Option<&HashMap<String, Parameters>>,
) -> Parameters {
    match (i, i2) {
        (Parameters::Null, Parameters::Int(v)) => Parameters::Int(v),
        (Parameters::Null, Parameters::Float(f)) => Parameters::Float(f),
        (Parameters::Int(v), Parameters::Null) => Parameters::Int(v),
        (Parameters::Float(f), Parameters::Null) => Parameters::Float(f),
        (Parameters::Int(v), Parameters::Int(v2)) => Parameters::Float((v as f64).powf(v2 as f64)),
        (Parameters::Int(v), Parameters::Float(f)) => Parameters::Float((v as f64).powf(f)),
        (Parameters::Float(v), Parameters::Float(f)) => Parameters::Float(v.powf(f)),
        (Parameters::Float(v), Parameters::Int(i1)) => Parameters::Float(v.powf(i1 as f64)),
        (Bool(_), Parameters::Int(i)) => Parameters::Int(i),
        (Bool(_), Parameters::Float(i)) => Parameters::Float(i),
        (Parameters::Int(i), Bool(_)) => Parameters::Int(i),
        (Parameters::Float(i), Bool(_)) => Parameters::Float(i),
        (Bool(b), Parameters::Null) => Bool(b),
        (Parameters::Null, Bool(b)) => Bool(b),
        (Bool(b), Bool(b2)) => Bool(b && b2),
        (Parameters::Identifier(s), Parameters::Identifier(s2)) => apply_operator(
            Parameters::Identifier(s),
            Parameters::Identifier(s2),
            ram,
            expo,
        ),
        (Parameters::Identifier(s), Parameters::Int(i)) => {
            apply_operator(Parameters::Identifier(s), Parameters::Int(i), ram, expo)
        }
        (Parameters::Int(i), Parameters::Identifier(s)) => {
            apply_operator_reverse(Parameters::Int(i), Parameters::Identifier(s), ram, expo)
        }
        (Parameters::Identifier(s), Parameters::Float(i)) => {
            apply_operator(Parameters::Identifier(s), Parameters::Float(i), ram, expo)
        }
        (Parameters::Identifier(s), Parameters::Null) => {
            apply_operator(Parameters::Identifier(s), Parameters::Null, ram, expo)
        }
        (Parameters::Null, Parameters::Identifier(s)) => {
            apply_operator(Parameters::Identifier(s), Parameters::Null, ram, expo)
        }
        (Parameters::Float(i), Parameters::Identifier(s)) => {
            apply_operator_reverse(Parameters::Float(i), Parameters::Identifier(s), ram, expo)
        }
        (Bool(b), Parameters::Identifier(s)) => {
            apply_operator_reverse(Bool(b), Parameters::Identifier(s), ram, expo)
        }
        (Parameters::Identifier(s), Bool(b)) => {
            apply_operator(Parameters::Identifier(s), Bool(b), ram, expo)
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

pub fn greater(
    i: Parameters,
    i2: Parameters,
    ram: Option<&HashMap<String, Parameters>>,
) -> Parameters {
    match (i, i2) {
        (Parameters::Null, Parameters::Int(_)) => Bool(true),
        (Parameters::Null, Parameters::Float(_)) => Bool(true),
        (Parameters::Int(_), Parameters::Null) => Bool(true),
        (Parameters::Float(_), Parameters::Null) => Bool(true),
        (Parameters::Int(v), Parameters::Int(v2)) => Bool(v > v2),
        (Parameters::Int(v), Parameters::Float(f)) => Bool((v as f64) > f),
        (Parameters::Float(v), Parameters::Float(f)) => Bool(v > f),
        (Parameters::Float(v), Parameters::Int(i1)) => Bool(v > (i1 as f64)),
        (Bool(b), Parameters::Int(_)) => Bool(b),
        (Bool(b), Parameters::Float(_)) => Bool(b),
        (Parameters::Int(_), Bool(b)) => Bool(b),
        (Parameters::Float(_), Bool(b)) => Bool(b),
        (Bool(b), Parameters::Null) => Bool(b),
        (Parameters::Null, Bool(b)) => Bool(b),
        (Bool(b), Bool(b2)) => Bool(b && b2),
        (Parameters::Identifier(s), Parameters::Identifier(s2)) => apply_operator(
            Parameters::Identifier(s),
            Parameters::Identifier(s2),
            ram,
            greater,
        ),
        (Parameters::Identifier(s), Parameters::Int(i)) => {
            apply_operator(Parameters::Identifier(s), Parameters::Int(i), ram, greater)
        }
        (Parameters::Null, Parameters::Identifier(s)) => {
            apply_operator(Parameters::Identifier(s), Parameters::Null, ram, greater)
        }
        (Parameters::Identifier(s), Parameters::Null) => {
            apply_operator(Parameters::Identifier(s), Parameters::Null, ram, greater)
        }
        (Parameters::Int(i), Parameters::Identifier(s)) => {
            apply_operator_reverse(Parameters::Int(i), Parameters::Identifier(s), ram, greater)
        }
        (Parameters::Identifier(s), Parameters::Float(i)) => apply_operator(
            Parameters::Identifier(s),
            Parameters::Float(i),
            ram,
            greater,
        ),
        (Parameters::Float(i), Parameters::Identifier(s)) => apply_operator_reverse(
            Parameters::Float(i),
            Parameters::Identifier(s),
            ram,
            greater,
        ),
        (Bool(b), Parameters::Identifier(s)) => {
            apply_operator_reverse(Bool(b), Parameters::Identifier(s), ram, greater)
        }
        (Parameters::Identifier(s), Bool(b)) => {
            apply_operator(Parameters::Identifier(s), Bool(b), ram, greater)
        }
        _ => Parameters::Null,
    }
}

pub fn lesser(
    i: Parameters,
    i2: Parameters,
    ram: Option<&HashMap<String, Parameters>>,
) -> Parameters {
    match (i, i2) {
        (Parameters::Null, Parameters::Int(_)) => Bool(false),
        (Parameters::Null, Parameters::Float(_)) => Bool(false),
        (Parameters::Int(_), Parameters::Null) => Bool(false),
        (Parameters::Float(_), Parameters::Null) => Bool(false),
        (Parameters::Int(v), Parameters::Int(v2)) => Bool(v < v2),
        (Parameters::Int(v), Parameters::Float(f)) => Bool((v as f64) < f),
        (Parameters::Float(v), Parameters::Float(f)) => Bool(v < f),
        (Parameters::Float(v), Parameters::Int(i1)) => Bool(v < (i1 as f64)),
        (Bool(b), Parameters::Int(_)) => Bool(b),
        (Bool(b), Parameters::Float(_)) => Bool(b),
        (Parameters::Int(_), Bool(b)) => Bool(b),
        (Parameters::Float(_), Bool(b)) => Bool(b),
        (Bool(b), Parameters::Null) => Bool(b),
        (Parameters::Null, Bool(b)) => Bool(b),
        (Bool(b), Bool(b2)) => Bool(b && b2),
        (Parameters::Identifier(s), Parameters::Identifier(s2)) => apply_operator(
            Parameters::Identifier(s),
            Parameters::Identifier(s2),
            ram,
            lesser,
        ),
        (Parameters::Identifier(s), Parameters::Int(i)) => {
            apply_operator(Parameters::Identifier(s), Parameters::Int(i), ram, lesser)
        }
        (Parameters::Null, Parameters::Identifier(s)) => {
            apply_operator(Parameters::Identifier(s), Parameters::Null, ram, lesser)
        }
        (Parameters::Identifier(s), Parameters::Null) => {
            apply_operator(Parameters::Identifier(s), Parameters::Null, ram, lesser)
        }
        (Parameters::Int(i), Parameters::Identifier(s)) => {
            apply_operator_reverse(Parameters::Int(i), Parameters::Identifier(s), ram, lesser)
        }
        (Parameters::Identifier(s), Parameters::Float(i)) => {
            apply_operator(Parameters::Identifier(s), Parameters::Float(i), ram, lesser)
        }
        (Parameters::Float(i), Parameters::Identifier(s)) => {
            apply_operator_reverse(Parameters::Float(i), Parameters::Identifier(s), ram, lesser)
        }
        (Bool(b), Parameters::Identifier(s)) => {
            apply_operator_reverse(Bool(b), Parameters::Identifier(s), ram, lesser)
        }
        (Parameters::Identifier(s), Bool(b)) => {
            apply_operator(Parameters::Identifier(s), Bool(b), ram, lesser)
        }
        _ => Parameters::Null,
    }
}

pub fn greater_or_equal(
    i: Parameters,
    i2: Parameters,
    ram: Option<&HashMap<String, Parameters>>,
) -> Parameters {
    match (i, i2) {
        (Parameters::Null, Parameters::Int(_)) => Bool(true),
        (Parameters::Null, Parameters::Float(_)) => Bool(true),
        (Parameters::Int(_), Parameters::Null) => Bool(true),
        (Parameters::Float(_), Parameters::Null) => Bool(true),
        (Parameters::Int(v), Parameters::Int(v2)) => Bool(v >= v2),
        (Parameters::Int(v), Parameters::Float(f)) => Bool((v as f64) >= f),
        (Parameters::Float(v), Parameters::Float(f)) => Bool(v >= f),
        (Parameters::Float(v), Parameters::Int(i1)) => Bool(v >= (i1 as f64)),
        (Bool(b), Parameters::Int(_)) => Bool(b),
        (Bool(b), Parameters::Float(_)) => Bool(b),
        (Parameters::Int(_), Bool(b)) => Bool(b),
        (Parameters::Float(_), Bool(b)) => Bool(b),
        (Bool(b), Parameters::Null) => Bool(b),
        (Parameters::Null, Bool(b)) => Bool(b),
        (Bool(b), Bool(b2)) => Bool(b == b2),
        (Parameters::Identifier(s), Parameters::Identifier(s2)) => apply_operator(
            Parameters::Identifier(s),
            Parameters::Identifier(s2),
            ram,
            greater_or_equal,
        ),
        (Parameters::Identifier(s), Parameters::Int(i)) => apply_operator(
            Parameters::Identifier(s),
            Parameters::Int(i),
            ram,
            greater_or_equal,
        ),
        (Parameters::Null, Parameters::Identifier(s)) => apply_operator(
            Parameters::Identifier(s),
            Parameters::Null,
            ram,
            greater_or_equal,
        ),
        (Parameters::Identifier(s), Parameters::Null) => apply_operator(
            Parameters::Identifier(s),
            Parameters::Null,
            ram,
            greater_or_equal,
        ),
        (Parameters::Int(i), Parameters::Identifier(s)) => apply_operator_reverse(
            Parameters::Int(i),
            Parameters::Identifier(s),
            ram,
            greater_or_equal,
        ),
        (Parameters::Identifier(s), Parameters::Float(i)) => apply_operator(
            Parameters::Identifier(s),
            Parameters::Float(i),
            ram,
            greater_or_equal,
        ),
        (Parameters::Float(i), Parameters::Identifier(s)) => apply_operator_reverse(
            Parameters::Float(i),
            Parameters::Identifier(s),
            ram,
            greater_or_equal,
        ),
        (Bool(b), Parameters::Identifier(s)) => {
            apply_operator_reverse(Bool(b), Parameters::Identifier(s), ram, greater_or_equal)
        }
        (Parameters::Identifier(s), Bool(b)) => {
            apply_operator(Parameters::Identifier(s), Bool(b), ram, greater_or_equal)
        }
        _ => Parameters::Null,
    }
}

pub fn lesser_or_equal(
    i: Parameters,
    i2: Parameters,
    ram: Option<&HashMap<String, Parameters>>,
) -> Parameters {
    match (i, i2) {
        (Parameters::Null, Parameters::Int(_)) => Bool(false),
        (Parameters::Null, Parameters::Float(_)) => Bool(false),
        (Parameters::Int(_), Parameters::Null) => Bool(false),
        (Parameters::Float(_), Parameters::Null) => Bool(false),
        (Parameters::Int(v), Parameters::Int(v2)) => Bool(v <= v2),
        (Parameters::Int(v), Parameters::Float(f)) => Bool((v as f64) <= f),
        (Parameters::Float(v), Parameters::Float(f)) => Bool(v <= f),
        (Parameters::Float(v), Parameters::Int(i1)) => Bool(v <= (i1 as f64)),
        (Bool(b), Parameters::Int(_)) => Bool(b),
        (Bool(b), Parameters::Float(_)) => Bool(b),
        (Parameters::Int(_), Bool(b)) => Bool(b),
        (Parameters::Float(_), Bool(b)) => Bool(b),
        (Bool(b), Parameters::Null) => Bool(b),
        (Parameters::Null, Bool(b)) => Bool(b),
        (Bool(b), Bool(b2)) => Bool(b == b2),

        (Parameters::Identifier(s), Parameters::Identifier(s2)) => apply_operator(
            Parameters::Identifier(s),
            Parameters::Identifier(s2),
            ram,
            lesser_or_equal,
        ),
        (Parameters::Identifier(s), Parameters::Int(i)) => apply_operator(
            Parameters::Identifier(s),
            Parameters::Int(i),
            ram,
            lesser_or_equal,
        ),
        (Parameters::Null, Parameters::Identifier(s)) => apply_operator(
            Parameters::Identifier(s),
            Parameters::Null,
            ram,
            lesser_or_equal,
        ),
        (Parameters::Identifier(s), Parameters::Null) => apply_operator(
            Parameters::Identifier(s),
            Parameters::Null,
            ram,
            lesser_or_equal,
        ),
        (Parameters::Int(i), Parameters::Identifier(s)) => apply_operator_reverse(
            Parameters::Int(i),
            Parameters::Identifier(s),
            ram,
            lesser_or_equal,
        ),
        (Parameters::Identifier(s), Parameters::Float(i)) => apply_operator(
            Parameters::Identifier(s),
            Parameters::Float(i),
            ram,
            lesser_or_equal,
        ),
        (Parameters::Float(i), Parameters::Identifier(s)) => apply_operator_reverse(
            Parameters::Float(i),
            Parameters::Identifier(s),
            ram,
            lesser_or_equal,
        ),
        (Bool(b), Parameters::Identifier(s)) => {
            apply_operator_reverse(Bool(b), Parameters::Identifier(s), ram, lesser_or_equal)
        }
        (Parameters::Identifier(s), Bool(b)) => {
            apply_operator(Parameters::Identifier(s), Bool(b), ram, lesser_or_equal)
        }
        _ => Parameters::Null,
    }
}

pub fn equal(
    i: Parameters,
    i2: Parameters,
    ram: Option<&HashMap<String, Parameters>>,
) -> Parameters {
    match (i, i2) {
        (Parameters::Null, Parameters::Int(_)) => Bool(true),
        (Parameters::Null, Parameters::Float(_)) => Bool(true),
        (Parameters::Int(_), Parameters::Null) => Bool(true),
        (Parameters::Float(_), Parameters::Null) => Bool(true),
        (Parameters::Int(v), Parameters::Int(v2)) => Bool(v == v2),
        (Parameters::Int(v), Parameters::Float(f)) => Bool((v as f64) == f),
        (Parameters::Float(v), Parameters::Float(f)) => Bool(v == f),
        (Parameters::Float(v), Parameters::Int(i1)) => Bool(v == (i1 as f64)),
        (Bool(_), Parameters::Int(_)) => Bool(false),
        (Bool(_), Parameters::Float(_)) => Bool(false),
        (Parameters::Int(_), Bool(_)) => Bool(false),
        (Parameters::Float(_), Bool(_)) => Bool(false),
        (Bool(_), Parameters::Null) => Bool(false),
        (Parameters::Null, Bool(_)) => Bool(false),
        (Bool(b), Bool(b2)) => Bool(b == b2),
        (Parameters::Identifier(s), Parameters::Identifier(s2)) => apply_operator(
            Parameters::Identifier(s),
            Parameters::Identifier(s2),
            ram,
            equal,
        ),
        (Parameters::Identifier(s), Parameters::Int(i)) => {
            apply_operator(Parameters::Identifier(s), Parameters::Int(i), ram, equal)
        }
        (Parameters::Null, Parameters::Identifier(s)) => {
            apply_operator(Parameters::Identifier(s), Parameters::Null, ram, equal)
        }
        (Parameters::Identifier(s), Parameters::Null) => {
            apply_operator(Parameters::Identifier(s), Parameters::Null, ram, equal)
        }
        (Parameters::Int(i), Parameters::Identifier(s)) => {
            apply_operator_reverse(Parameters::Int(i), Parameters::Identifier(s), ram, equal)
        }
        (Parameters::Identifier(s), Parameters::Float(i)) => {
            apply_operator(Parameters::Identifier(s), Parameters::Float(i), ram, equal)
        }
        (Parameters::Float(i), Parameters::Identifier(s)) => {
            apply_operator_reverse(Parameters::Float(i), Parameters::Identifier(s), ram, equal)
        }
        (Bool(b), Parameters::Identifier(s)) => {
            apply_operator_reverse(Bool(b), Parameters::Identifier(s), ram, equal)
        }
        (Parameters::Identifier(s), Bool(b)) => {
            apply_operator(Parameters::Identifier(s), Bool(b), ram, equal)
        }
        _ => Parameters::Null,
    }
}

pub fn not(
    i: Parameters,
    _i2: Parameters,
    ram: Option<&HashMap<String, Parameters>>,
) -> Parameters {
    match i {
        Bool(b) => Bool(!b),
        Parameters::Identifier(s) => {
            apply_operator(Parameters::Identifier(s), Parameters::Null, ram, not)
        }
        _ => Bool(false),
    }
}

pub fn and(i: Parameters, i2: Parameters, ram: Option<&HashMap<String, Parameters>>) -> Parameters {
    match (i, i2) {
        (Bool(b), Bool(b2)) => Bool(b && b2),
        (Bool(b), Parameters::Null) => Bool(b),
        (Parameters::Null, Bool(b)) => Bool(b),
        (Parameters::Identifier(s), Bool(b)) => {
            apply_operator(Parameters::Identifier(s), Bool(b), ram, and)
        }
        (Bool(b), Parameters::Identifier(s)) => {
            apply_operator_reverse(Bool(b), Parameters::Identifier(s), ram, and)
        }
        (Parameters::Identifier(s), Parameters::Identifier(s2)) => apply_operator(
            Parameters::Identifier(s),
            Parameters::Identifier(s2),
            ram,
            and,
        ),
        _ => Bool(false),
    }
}

pub fn or(i: Parameters, i2: Parameters, ram: Option<&HashMap<String, Parameters>>) -> Parameters {
    match (i, i2) {
        (Bool(b), Bool(b2)) => Bool(b || b2),
        (Bool(b), Parameters::Null) => Bool(b),
        (Parameters::Null, Bool(b)) => Bool(b),
        (Parameters::Identifier(s), Bool(b)) => {
            apply_operator(Parameters::Identifier(s), Bool(b), ram, or)
        }
        (Bool(b), Parameters::Identifier(s)) => {
            apply_operator_reverse(Bool(b), Parameters::Identifier(s), ram, or)
        }
        (Parameters::Identifier(s), Parameters::Identifier(s2)) => apply_operator(
            Parameters::Identifier(s),
            Parameters::Identifier(s2),
            ram,
            or,
        ),
        _ => Bool(false),
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
