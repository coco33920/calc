use std::collections::HashMap;
use std::f64::consts::PI;

use crate::parsing::ast::Parameters;

pub fn exec(
    s: String,
    lst: Vec<Parameters>,
    ram: Option<&HashMap<String, Parameters>>,
) -> Parameters {
    match s.as_str() {
        "cos" => cos(&lst, ram),
        "sin" => sin(&lst, ram),
        "tan" => tan(&lst, ram),
        "cosh" => cosh(&lst, ram),
        "sinh" => sinh(&lst, ram),
        "tanh" => tanh(&lst, ram),
        "exp" => exp(&lst, ram),
        "acos" => acos(&lst, ram),
        "asin" => asin(&lst, ram),
        "atan" => atan(&lst, ram),
        "ln" => ln(&lst, ram),
        "log" => ln(&lst, ram),
        "sqrt" => sqrt(&lst, ram),
        "!" => factorial(&lst, ram),
        "fact" => factorial(&lst, ram),
        "factorial" => factorial(&lst, ram),
        "abs" => abs(&lst, ram),
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

pub fn tan(p: &Vec<Parameters>, ram: Option<&HashMap<String, Parameters>>) -> Parameters {
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
            Parameters::Float(fs.tan())
        }
        Parameters::Float(f) => {
            let fs: f64 = if degrees { (*f) * (PI / 180.0) } else { *f };
            Parameters::Float(fs.tan())
        }
        Parameters::Identifier(s) => match ram {
            None => Parameters::Identifier("This variable is not initialized yet".to_string()),
            Some(t) => match t.get(s.as_str()) {
                None => Parameters::Null,
                Some(t) => tan(&vec![t.clone()], ram),
            },
        },
        _ => Parameters::Null,
    }
}

pub fn cosh(p: &Vec<Parameters>, ram: Option<&HashMap<String, Parameters>>) -> Parameters {
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
            Parameters::Float(fs.cosh())
        }
        Parameters::Float(f) => {
            let fs: f64 = if degrees { (*f) * (PI / 180.0) } else { *f };
            Parameters::Float(fs.cosh())
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

pub fn sinh(p: &Vec<Parameters>, ram: Option<&HashMap<String, Parameters>>) -> Parameters {
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
            Parameters::Float(fs.sinh())
        }
        Parameters::Float(f) => {
            let fs: f64 = if degrees { (*f) * (PI / 180.0) } else { *f };
            Parameters::Float(fs.sinh())
        }
        Parameters::Identifier(s) => match ram {
            None => Parameters::Identifier("This variable is not initialized yet".to_string()),
            Some(t) => match t.get(s.as_str()) {
                None => Parameters::Null,
                Some(t) => sinh(&vec![t.clone()], ram),
            },
        },
        _ => Parameters::Null,
    }
}

pub fn tanh(p: &Vec<Parameters>, ram: Option<&HashMap<String, Parameters>>) -> Parameters {
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
            Parameters::Float(fs.tanh())
        }
        Parameters::Float(f) => {
            let fs: f64 = if degrees { (*f) * (PI / 180.0) } else { *f };
            Parameters::Float(fs.tanh())
        }
        Parameters::Identifier(s) => match ram {
            None => Parameters::Identifier("This variable is not initialized yet".to_string()),
            Some(t) => match t.get(s.as_str()) {
                None => Parameters::Null,
                Some(t) => tanh(&vec![t.clone()], ram),
            },
        },
        _ => Parameters::Null,
    }
}

pub fn acos(p: &Vec<Parameters>, ram: Option<&HashMap<String, Parameters>>) -> Parameters {
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
            let fs: f64 = (*i) as f64;
            Parameters::Float(if degrees {
                fs.acos() * (180.0 / PI)
            } else {
                fs.acos()
            })
        }
        Parameters::Float(f) => Parameters::Float(if degrees {
            f.acos() * (180.0 / PI)
        } else {
            f.acos()
        }),
        Parameters::Identifier(s) => match ram {
            None => Parameters::Identifier("This variable is not initialized yet".to_string()),
            Some(t) => match t.get(s.as_str()) {
                None => Parameters::Null,
                Some(t) => acos(&vec![t.clone()], ram),
            },
        },
        _ => Parameters::Null,
    }
}

pub fn asin(p: &Vec<Parameters>, ram: Option<&HashMap<String, Parameters>>) -> Parameters {
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
            let fs: f64 = (*i) as f64;
            Parameters::Float(if degrees {
                fs.asin() * (180.0 / PI)
            } else {
                fs.asin()
            })
        }
        Parameters::Float(f) => Parameters::Float(if degrees {
            f.asin() * (180.0 / PI)
        } else {
            f.asin()
        }),
        Parameters::Identifier(s) => match ram {
            None => Parameters::Identifier("This variable is not initialized yet".to_string()),
            Some(t) => match t.get(s.as_str()) {
                None => Parameters::Null,
                Some(t) => asin(&vec![t.clone()], ram),
            },
        },
        _ => Parameters::Null,
    }
}

pub fn atan(p: &Vec<Parameters>, ram: Option<&HashMap<String, Parameters>>) -> Parameters {
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
            let fs: f64 = (*i) as f64;
            Parameters::Float(if degrees {
                fs.atan() * (180.0 / PI)
            } else {
                fs.atan()
            })
        }
        Parameters::Float(f) => Parameters::Float(if degrees {
            f.atan() * (180.0 / PI)
        } else {
            f.atan()
        }),
        Parameters::Identifier(s) => match ram {
            None => Parameters::Identifier("This variable is not initialized yet".to_string()),
            Some(t) => match t.get(s.as_str()) {
                None => Parameters::Null,
                Some(t) => atan(&vec![t.clone()], ram),
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
                    Parameters::Float(f) => ln = *f,
                    Parameters::Int(i) => ln = (*i) as f64,
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

pub fn ln(p: &Vec<Parameters>, ram: Option<&HashMap<String, Parameters>>) -> Parameters {
    if p.len() < 1 {
        return Parameters::Null;
    }

    let mut plus = false;
    let mut sln: f64 = 0.0;

    if p.len() > 1 {
        match p.get(1) {
            None => plus = false,
            Some(t) => {
                plus = true;
                match t {
                    Parameters::Float(f) => sln = *f,
                    Parameters::Int(i) => sln = (*i) as f64,
                    _ => sln = 0.0,
                }
            }
        }
    }

    match p.get(0).unwrap() {
        Parameters::Int(i) => {
            let fs: f64 = (*i) as f64;
            if plus {
                Parameters::Float(fs.log(sln))
            } else {
                Parameters::Float(fs.ln())
            }
        }
        Parameters::Float(f) => {
            if plus {
                Parameters::Float((*f).log(sln))
            } else {
                Parameters::Float((*f).ln())
            }
        }
        Parameters::Identifier(s) => match ram {
            None => Parameters::Identifier("This variable is not initialized yet".to_string()),
            Some(t) => match t.get(s.as_str()) {
                None => Parameters::Null,
                Some(t) => ln(&vec![t.clone()], ram),
            },
        },
        _ => Parameters::Null,
    }
}

pub fn sqrt(p: &Vec<Parameters>, ram: Option<&HashMap<String, Parameters>>) -> Parameters {
    if p.len() < 1 {
        return Parameters::Null;
    }

    let mut plus = false;
    let mut sln: f64 = 0.0;

    if p.len() > 1 {
        match p.get(1) {
            None => plus = false,
            Some(t) => {
                plus = true;
                match t {
                    Parameters::Float(f) => sln = *f,
                    Parameters::Int(i) => sln = (*i) as f64,
                    _ => sln = 0.0,
                }
            }
        }
    }

    match p.get(0).unwrap() {
        Parameters::Int(i) => {
            let fs: f64 = (*i) as f64;
            if plus {
                Parameters::Float(fs.powf(1.0 / sln))
            } else {
                Parameters::Float(fs.sqrt())
            }
        }
        Parameters::Float(f) => {
            if plus {
                Parameters::Float((*f).powf(1.0 / sln))
            } else {
                Parameters::Float((*f).sqrt())
            }
        }
        Parameters::Identifier(s) => match ram {
            None => Parameters::Identifier("This variable is not initialized yet".to_string()),
            Some(t) => match t.get(s.as_str()) {
                None => Parameters::Null,
                Some(t) => sqrt(&vec![t.clone()], ram),
            },
        },
        _ => Parameters::Null,
    }
}

pub fn fact(n: i64) -> i64 {
    fn aux(n: i64, acc: i64) -> i64 {
        match n {
            0 => acc,
            i => aux(i - 1, i * acc),
        }
    }
    aux(n, 1)
}

pub fn factorial(p: &Vec<Parameters>, ram: Option<&HashMap<String, Parameters>>) -> Parameters {
    if p.len() < 1 {
        return Parameters::Null;
    }

    match p.get(0).unwrap() {
        Parameters::Int(i) => Parameters::Int(fact(*i)),
        Parameters::Float(f) => Parameters::Int(fact(*f as i64)),
        Parameters::Identifier(s) => match ram {
            None => Parameters::Identifier("This variable is not initialized yet".to_string()),
            Some(t) => match t.get(s.as_str()) {
                None => Parameters::Null,
                Some(t) => factorial(&vec![t.clone()], ram),
            },
        },
        _ => Parameters::Null,
    }
}

pub fn abs(p: &Vec<Parameters>, ram: Option<&HashMap<String, Parameters>>) -> Parameters {
    if p.len() < 1 {
        return Parameters::Null;
    }

    match p.get(0).unwrap() {
        Parameters::Int(i) => Parameters::Int(i.abs()),
        Parameters::Float(f) => Parameters::Float(f.abs()),
        Parameters::Identifier(s) => match ram {
            None => Parameters::Identifier("This variable is not initialized yet".to_string()),
            Some(t) => match t.get(s.as_str()) {
                None => Parameters::Null,
                Some(t) => abs(&vec![t.clone()], ram),
            },
        },
        _ => Parameters::Null,
    }
}
