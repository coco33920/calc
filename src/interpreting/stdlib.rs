use crate::parsing::ast::{Ast, Parameters};
use std::collections::HashMap;

pub fn exec(s: String, p: Parameters, ram: Option<&HashMap<String, Parameters>>) -> Parameters {
    match s {
        _ => cos(p, ram),
    }
}

pub fn cos(p: Parameters, ram: Option<&HashMap<String, Parameters>>) -> Parameters {
    match p {
        Parameters::Int(i) => Parameters::Float((i.clone() as f64).cos()),
        Parameters::Float(f) => Parameters::Float(f.clone().cos()),
        Parameters::Identifier(s) => match ram {
            None => Parameters::Null,
            Some(i_ram) => match i_ram.get(s.as_str()) {
                None => Parameters::Null,
                Some(t) => cos(t.clone(), ram),
            },
        },
        _ => Parameters::Null,
    }
}
