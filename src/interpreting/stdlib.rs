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
        "exp" => exp(&lst, ram),
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

pub fn exp(p: &Vec<Parameters>, ram: Option<&HashMap<String, Parameters>>) -> Parameters {
    if p.len() < 1 {
        return Parameters::Null;
    }

    let mut plus = false;
    let mut ln: f64 = 0.0;

    if p.len() > 1 {
        match p.get(1) {
            None => plus = false,
            Some(t) => {
                plus = true;
                match t {
                    Parameters::Float(f) => ln = (*f),
                    Parameters::Int(i) => ln = ((*i) as f64),
                    _ => ln = 0.0,
                }
            }
        }
    }

    match p.get(0).unwrap() {
        Parameters::Int(i) => {
            let fs: f64 = (*i) as f64;
            if plus {
                Parameters::Float(ln.powf(fs))
            } else {
                Parameters::Float(fs.exp())
            }
        }
        Parameters::Float(f) => {
            if plus {
                Parameters::Float(ln.powf(*f))
            } else {
                Parameters::Float((*f).exp())
            }
        }
        Parameters::Identifier(s) => match ram {
            None => Parameters::Identifier("This variable is not initialized yet".to_string()),
            Some(t) => match t.get(s.as_str()) {
                None => Parameters::Null,
                Some(t) => exp(&vec![t.clone()], ram),
            },
        },
        _ => Parameters::Null,
    }
}
