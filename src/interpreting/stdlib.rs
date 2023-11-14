use crate::parsing::ast::Parameters;
use std::collections::HashMap;
use std::f64::consts::PI;

pub fn exec(
    s: String,
    lst: Vec<Parameters>,
    ram: Option<&HashMap<String, Parameters>>,
) -> Parameters {
    match s.as_str() {
        "cos" => cos(&lst, ram),
        "sin" => sin(&lst, ram),
        _ => cos(&lst, ram),
    }
}

pub fn cos(p: &Vec<Parameters>, ram: Option<&HashMap<String, Parameters>>) -> Parameters {
    if p.len() < 1 {
        return Parameters::Null;
    }

    let mut degrees = false;

    if p.len() > 1 {
        match p.get(1) {
            None => degrees = false,
            Some(_) => degrees = true,
        }
    }

    match p.get(0).unwrap() {
        Parameters::Int(i) => {
            let fs: f64 = if degrees {
                ((*i).clone() as f64) * (PI / 180.0)
            } else {
                (*i).clone() as f64
            };
            Parameters::Float(fs.cos())
        }
        Parameters::Float(f) => {
            let fs: f64 = if degrees { (*f) * (PI / 180.0) } else { *f };
            Parameters::Float(fs.cos())
        }
        Parameters::Identifier(s) => match ram {
            None => Parameters::Identifier("This variable is not initialized yet".to_string()),
            Some(t) => match t.get(s.as_str()) {
                None => Parameters::Null,
                Some(t) => cos(&vec![t.clone()], ram),
            },
        },
        _ => Parameters::Null,
    }
}

pub fn sin(p: &Vec<Parameters>, ram: Option<&HashMap<String, Parameters>>) -> Parameters {
    if p.len() < 1 {
        return Parameters::Null;
    }

    let mut degrees = false;

    if p.len() > 1 {
        match p.get(1) {
            None => degrees = false,
            Some(_) => degrees = true,
        }
    }

    match p.get(0).unwrap() {
        Parameters::Int(i) => {
            let fs: f64 = if degrees {
                ((*i).clone() as f64) * (PI / 180.0)
            } else {
                (*i).clone() as f64
            };
            Parameters::Float(fs.sin())
        }
        Parameters::Float(f) => {
            let fs: f64 = if degrees { (*f) * (PI / 180.0) } else { *f };
            Parameters::Float(fs.sin())
        }
        Parameters::Identifier(s) => match ram {
            None => Parameters::Identifier("This variable is not initialized yet".to_string()),
            Some(t) => match t.get(s.as_str()) {
                None => Parameters::Null,
                Some(t) => sin(&vec![t.clone()], ram),
            },
        },
        _ => Parameters::Null,
    }
}
