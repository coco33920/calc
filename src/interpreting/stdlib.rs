use std::collections::HashMap;
use std::f64::consts::{E, PI};

use crate::interpreting::interpreter::interpret;
use crate::parsing::ast::{Ast, Parameters};
use crate::utils::matrix_utils::{lup_decompose, lup_determinant, lup_invert, transpose};

use super::function::{add as other_add, mult};

pub fn exec(
    s: String,
    lst: Vec<Parameters>,
    ram: Option<&mut HashMap<String, Parameters>>,
    functions: Option<&mut HashMap<String, (Vec<Ast>, Ast)>>,
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
        "fact" => factorial(&lst, ram),
        "factorial" => factorial(&lst, ram),
        "abs" => abs(&lst, ram),
        "ceil" => ceil(&lst, ram),
        "floor" => floor(&lst, ram),
        "round" => round(&lst, ram),
        "norm" => norm(&lst, ram, functions),
        "transpose_vector" => transpose_vectors(&lst, ram),
        "transpose" => transpose_matrices(&lst, ram),
        "det" => det_matrix(&lst, ram),
        "invert" => inverse_matrix(&lst, ram),
        s => {
            let mut sram: HashMap<String, Parameters> = HashMap::new();
            sram.insert("pi".to_string(), Parameters::Float(PI));
            sram.insert("e".to_string(), Parameters::Float(E));
            match functions.cloned() {
                None => Parameters::Identifier("This function is unknown".to_string()),
                Some(mut f) => {
                    let fs = f.get_mut(s);
                    let (vec, ast): (&mut Vec<Ast>, &mut Ast) = match fs {
                        None => {
                            return Parameters::Identifier("This function is unknown".to_string());
                        }
                        Some((a, b)) => (a, b),
                    };
                    let mut names = Vec::new();
                    for v in vec {
                        match v {
                            Ast::Nil => (),
                            Ast::Call { .. } => (),
                            Ast::Node {
                                value: v,
                                left: _l,
                                right: _r,
                            } => match v {
                                Parameters::Identifier(s) => names.push(s.clone()),
                                _ => (),
                            },
                        }
                    }
                    names.iter().zip(lst).for_each(|(name, param)| {
                        sram.insert(name.to_string(), param);
                    });
                    interpret(ast, &mut sram, &mut HashMap::new())
                }
            }
        }
    }
}

pub fn cos(p: &Vec<Parameters>, ram: Option<&mut HashMap<String, Parameters>>) -> Parameters {
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
            Some(ref t) => match t.get(s.as_str()) {
                None => Parameters::Null,
                Some(t) => {
                    if degrees {
                        cos(
                            &vec![t.clone(), Parameters::Identifier("false".to_string())],
                            ram,
                        )
                    } else {
                        cos(&vec![t.clone()], ram)
                    }
                }
            },
        },
        _ => Parameters::Null,
    }
}

pub fn sin(p: &Vec<Parameters>, ram: Option<&mut HashMap<String, Parameters>>) -> Parameters {
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
            Some(ref t) => match t.get(s.as_str()) {
                None => Parameters::Null,
                Some(t) => {
                    if degrees {
                        sin(
                            &vec![t.clone(), Parameters::Identifier("false".to_string())],
                            ram,
                        )
                    } else {
                        sin(&vec![t.clone()], ram)
                    }
                }
            },
        },
        _ => Parameters::Null,
    }
}

pub fn tan(p: &Vec<Parameters>, ram: Option<&mut HashMap<String, Parameters>>) -> Parameters {
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
            Some(ref t) => match t.get(s.as_str()) {
                None => Parameters::Null,
                Some(t) => {
                    if degrees {
                        tan(
                            &vec![t.clone(), Parameters::Identifier("false".to_string())],
                            ram,
                        )
                    } else {
                        tan(&vec![t.clone()], ram)
                    }
                }
            },
        },
        _ => Parameters::Null,
    }
}

pub fn cosh(p: &Vec<Parameters>, ram: Option<&mut HashMap<String, Parameters>>) -> Parameters {
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
            Some(ref t) => match t.get(s.as_str()) {
                None => Parameters::Null,
                Some(t) => {
                    if degrees {
                        cosh(
                            &vec![t.clone(), Parameters::Identifier("false".to_string())],
                            ram,
                        )
                    } else {
                        cosh(&vec![t.clone()], ram)
                    }
                }
            },
        },
        _ => Parameters::Null,
    }
}

pub fn sinh(p: &Vec<Parameters>, ram: Option<&mut HashMap<String, Parameters>>) -> Parameters {
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
            Some(ref t) => match t.get(s.as_str()) {
                None => Parameters::Null,
                Some(t) => {
                    if degrees {
                        sinh(
                            &vec![t.clone(), Parameters::Identifier("false".to_string())],
                            ram,
                        )
                    } else {
                        sinh(&vec![t.clone()], ram)
                    }
                }
            },
        },
        _ => Parameters::Null,
    }
}

pub fn tanh(p: &Vec<Parameters>, ram: Option<&mut HashMap<String, Parameters>>) -> Parameters {
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
            Some(ref t) => match t.get(s.as_str()) {
                None => Parameters::Null,
                Some(t) => {
                    if degrees {
                        tanh(
                            &vec![t.clone(), Parameters::Identifier("false".to_string())],
                            ram,
                        )
                    } else {
                        tanh(&vec![t.clone()], ram)
                    }
                }
            },
        },
        _ => Parameters::Null,
    }
}

pub fn acos(p: &Vec<Parameters>, ram: Option<&mut HashMap<String, Parameters>>) -> Parameters {
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
            Some(ref t) => match t.get(s.as_str()) {
                None => Parameters::Null,
                Some(t) => {
                    if degrees {
                        acos(
                            &vec![t.clone(), Parameters::Identifier("false".to_string())],
                            ram,
                        )
                    } else {
                        acos(&vec![t.clone()], ram)
                    }
                }
            },
        },
        _ => Parameters::Null,
    }
}

pub fn asin(p: &Vec<Parameters>, ram: Option<&mut HashMap<String, Parameters>>) -> Parameters {
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
            Some(ref t) => match t.get(s.as_str()) {
                None => Parameters::Null,
                Some(t) => {
                    if degrees {
                        asin(
                            &vec![t.clone(), Parameters::Identifier("false".to_string())],
                            ram,
                        )
                    } else {
                        asin(&vec![t.clone()], ram)
                    }
                }
            },
        },
        _ => Parameters::Null,
    }
}

pub fn atan(p: &Vec<Parameters>, ram: Option<&mut HashMap<String, Parameters>>) -> Parameters {
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
            Some(ref t) => match t.get(s.as_str()) {
                None => Parameters::Null,
                Some(t) => {
                    if degrees {
                        atan(
                            &vec![t.clone(), Parameters::Identifier("false".to_string())],
                            ram,
                        )
                    } else {
                        atan(&vec![t.clone()], ram)
                    }
                }
            },
        },
        _ => Parameters::Null,
    }
}

pub fn exp(p: &Vec<Parameters>, ram: Option<&mut HashMap<String, Parameters>>) -> Parameters {
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
            Some(ref t) => match t.get(s.as_str()) {
                None => Parameters::Null,
                Some(t) => exp(&vec![t.clone(), Parameters::Float(ln)], ram),
            },
        },
        _ => Parameters::Null,
    }
}

pub fn ln(p: &Vec<Parameters>, ram: Option<&mut HashMap<String, Parameters>>) -> Parameters {
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
            Some(ref t) => match t.get(s.as_str()) {
                None => Parameters::Null,
                Some(t) => ln(&vec![t.clone(), Parameters::Float(sln)], ram),
            },
        },
        _ => Parameters::Null,
    }
}

pub fn sqrt(p: &Vec<Parameters>, ram: Option<&mut HashMap<String, Parameters>>) -> Parameters {
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
            Some(ref t) => match t.get(s.as_str()) {
                None => Parameters::Null,
                Some(t) => sqrt(&vec![t.clone(), Parameters::Float(sln)], ram),
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

pub fn factorial(p: &Vec<Parameters>, ram: Option<&mut HashMap<String, Parameters>>) -> Parameters {
    if p.len() < 1 {
        return Parameters::Null;
    }

    match p.get(0).unwrap() {
        Parameters::Int(i) => Parameters::Int(fact(*i)),
        Parameters::Float(f) => Parameters::Int(fact(*f as i64)),
        Parameters::Identifier(s) => match ram {
            None => Parameters::Identifier("This variable is not initialized yet".to_string()),
            Some(ref t) => match t.get(s.as_str()) {
                None => Parameters::Null,
                Some(t) => factorial(&vec![t.clone()], ram),
            },
        },
        _ => Parameters::Null,
    }
}

pub fn abs(p: &Vec<Parameters>, ram: Option<&mut HashMap<String, Parameters>>) -> Parameters {
    if p.len() < 1 {
        return Parameters::Null;
    }

    match p.get(0).unwrap() {
        Parameters::Int(i) => Parameters::Int(i.abs()),
        Parameters::Float(f) => Parameters::Float(f.abs()),
        Parameters::Identifier(s) => match ram {
            None => Parameters::Identifier("This variable is not initialized yet".to_string()),
            Some(ref t) => match t.get(s.as_str()) {
                None => Parameters::Null,
                Some(t) => abs(&vec![t.clone()], ram),
            },
        },
        _ => Parameters::Null,
    }
}

pub fn ceil(p: &Vec<Parameters>, ram: Option<&mut HashMap<String, Parameters>>) -> Parameters {
    if p.len() < 1 {
        return Parameters::Null;
    }

    match p.get(0).unwrap() {
        Parameters::Int(i) => Parameters::Float((*i as f64).ceil()),
        Parameters::Float(f) => Parameters::Float(f.ceil()),
        Parameters::Identifier(s) => match ram {
            None => Parameters::Identifier("This variable is not initialized yet".to_string()),
            Some(ref t) => match t.get(s.as_str()) {
                None => Parameters::Null,
                Some(t) => ceil(&vec![t.clone()], ram),
            },
        },
        _ => Parameters::Null,
    }
}

pub fn floor(p: &Vec<Parameters>, ram: Option<&mut HashMap<String, Parameters>>) -> Parameters {
    if p.len() < 1 {
        return Parameters::Null;
    }

    match p.get(0).unwrap() {
        Parameters::Int(i) => Parameters::Float((*i as f64).floor()),
        Parameters::Float(f) => Parameters::Float(f.floor()),
        Parameters::Identifier(s) => match ram {
            None => Parameters::Identifier("This variable is not initialized yet".to_string()),
            Some(ref t) => match t.get(s.as_str()) {
                None => Parameters::Null,
                Some(t) => floor(&vec![t.clone()], ram),
            },
        },
        _ => Parameters::Null,
    }
}

pub fn round(p: &Vec<Parameters>, ram: Option<&mut HashMap<String, Parameters>>) -> Parameters {
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
                Parameters::Float(((fs * 10.0_f64.powf(sln)).round()) / (10.0_f64.powf(sln)))
            } else {
                Parameters::Float(fs.round())
            }
        }
        Parameters::Float(f) => {
            if plus {
                Parameters::Float(((f * 10.0_f64.powf(sln)).round()) / (10.0_f64.powf(sln)))
            } else {
                Parameters::Float((*f).round())
            }
        }
        Parameters::Identifier(s) => match ram {
            None => Parameters::Identifier("This variable is not initialized yet".to_string()),
            Some(ref t) => match t.get(s.as_str()) {
                None => Parameters::Null,
                Some(t) => round(&vec![t.clone(), Parameters::Float(sln)], ram),
            },
        },
        _ => Parameters::Null,
    }
}

pub fn norm(
    p: &Vec<Parameters>,
    ram: Option<&mut HashMap<String, Parameters>>,
    function: Option<&mut HashMap<String, (Vec<Ast>, Ast)>>,
) -> Parameters {
    if p.len() < 1 {
        return Parameters::Null;
    }

    match p.get(0).unwrap() {
        Parameters::Int(i) => Parameters::Int((*i).abs()),
        Parameters::Float(f) => Parameters::Float((*f).abs()),
        Parameters::InterpreterVector(lst) => {
            let mut sum = Parameters::Int(0);

            (*lst)
                .iter()
                .map(|x| mult(x.clone(), x.clone(), ram.as_deref()))
                .for_each(|x| sum = other_add(sum.clone(), x.clone(), ram.as_deref()));

            match sum {
                Parameters::Int(i) => Parameters::Float((i as f64).sqrt()),
                Parameters::Float(f) => Parameters::Float(f.sqrt()),
                _ => Parameters::Float(0.0),
            }
        }
        Parameters::Identifier(s) => match ram {
            None => Parameters::Identifier("This variable is not initialized yet".to_string()),
            Some(ref t) => match t.get(s.as_str()) {
                None => Parameters::Null,
                Some(t) => norm(&vec![t.clone()], ram, function),
            },
        },
        _ => Parameters::Null,
    }
}

pub fn transpose_vectors(
    p: &Vec<Parameters>,
    ram: Option<&mut HashMap<String, Parameters>>,
) -> Parameters {
    if p.len() < 1 {
        return Parameters::Null;
    }

    match p.get(0).unwrap() {
        Parameters::Int(i) => Parameters::Int((*i).abs()),
        Parameters::Float(f) => Parameters::Float((*f).abs()),
        Parameters::InterpreterVector(lst) => {
            let r = vec![*(lst.clone())];
            let transposed = transpose(r);

            let mut result = Vec::new();

            transposed
                .into_iter()
                .map(|v| Parameters::InterpreterVector(Box::from(v)))
                .for_each(|v| result.push(v));

            Parameters::InterpreterVector(Box::from(result))
        }
        Parameters::Identifier(s) => match ram {
            None => Parameters::Identifier("This variable is not initialized yet".to_string()),
            Some(ref t) => match t.get(s.as_str()) {
                None => Parameters::Null,
                Some(t) => transpose_vectors(&vec![t.clone()], ram),
            },
        },
        _ => Parameters::Null,
    }
}

pub fn transpose_matrices(
    p: &Vec<Parameters>,
    ram: Option<&mut HashMap<String, Parameters>>,
) -> Parameters {
    if p.len() < 1 {
        return Parameters::Null;
    }

    match p.get(0).unwrap() {
        Parameters::Int(i) => Parameters::Int((*i).abs()),
        Parameters::Float(f) => Parameters::Float((*f).abs()),
        Parameters::InterpreterVector(lst) => {
            let mut res1 = Vec::new();
            let mut is_matrix = true;
            let mut res = Vec::new();
            lst.clone().into_iter().for_each(|x| match x {
                Parameters::InterpreterVector(l) => res.push(l.to_vec()),
                p => {
                    is_matrix = false;
                    res1.push(p);
                }
            });

            if !is_matrix {
                return transpose_vectors(p, ram);
            }

            let matrix_result = transpose(res);
            let mut result = Vec::new();

            matrix_result
                .into_iter()
                .for_each(|x| result.push(Parameters::InterpreterVector(Box::from(x))));
            Parameters::InterpreterVector(Box::from(result))
        }

        Parameters::Identifier(s) => match ram {
            None => Parameters::Identifier("This variable is not initialized yet".to_string()),
            Some(ref t) => match t.get(s.as_str()) {
                None => Parameters::Null,
                Some(t) => transpose_matrices(&vec![t.clone()], ram),
            },
        },
        _ => Parameters::Null,
    }
}

pub fn det_matrix(
    p: &Vec<Parameters>,
    ram: Option<&mut HashMap<String, Parameters>>,
) -> Parameters {
    if p.len() < 1 {
        return Parameters::Null;
    }

    match p.get(0).unwrap() {
        Parameters::Int(i) => Parameters::Int((*i).abs()),
        Parameters::Float(f) => Parameters::Float((*f).abs()),
        Parameters::InterpreterVector(lst) => {
            let mut res1 = Vec::new();
            let mut is_matrix = true;
            let mut res = Vec::new();
            lst.clone().into_iter().for_each(|x| match x {
                Parameters::InterpreterVector(l) => res.push(l.to_vec()),
                p => {
                    is_matrix = false;
                    res1.push(p);
                }
            });

            if !is_matrix {
                return Parameters::Float(0.0);
            }

            let mut p = Vec::new();
            for _ in 0..(res.len() + 1) {
                p.push(Parameters::Int(0));
            }
            let n = res.len();
            let r = lup_decompose(&mut res, &mut p, n, ram.as_deref());

            match r {
                0 => Parameters::Int(0),
                _ => {
                    let det = lup_determinant(&mut res, &mut p, n, ram.as_deref());
                    det
                }
            }
        }

        Parameters::Identifier(s) => match ram {
            None => Parameters::Identifier("This variable is not initialized yet".to_string()),
            Some(ref t) => match t.get(s.as_str()) {
                None => Parameters::Null,
                Some(t) => det_matrix(&vec![t.clone()], ram),
            },
        },
        _ => Parameters::Null,
    }
}

pub fn inverse_matrix(
    p: &Vec<Parameters>,
    ram: Option<&mut HashMap<String, Parameters>>,
) -> Parameters {
    if p.len() < 1 {
        return Parameters::Null;
    }

    match p.get(0).unwrap() {
        Parameters::Int(i) => Parameters::Int((*i).abs()),
        Parameters::Float(f) => Parameters::Float((*f).abs()),
        Parameters::InterpreterVector(lst) => {
            let mut res1 = Vec::new();
            let mut is_matrix = true;
            let mut res = Vec::new();
            lst.clone().into_iter().for_each(|x| match x {
                Parameters::InterpreterVector(l) => res.push(l.to_vec()),
                p => {
                    is_matrix = false;
                    res1.push(p);
                }
            });

            if !is_matrix {
                return Parameters::InterpreterVector(Box::from(res1));
            }

            let mut p = Vec::new();
            for _ in 0..(res.len() + 1) {
                p.push(Parameters::Int(0));
            }
            let n = res.len();
            let r = lup_decompose(&mut res, &mut p, n, ram.as_deref());

            match r {
                0 => Parameters::Null,
                _ => {
                    let mut vec_ia = Vec::new();
                    for _ in 0..n {
                        let mut vec = Vec::new();
                        for _ in 0..n {
                            vec.push(Parameters::Int(0));
                        }
                        vec_ia.push(vec);
                    }
                    lup_invert(&mut res, &mut p, n, &mut vec_ia, ram.as_deref());
                    let mut resd = Vec::new();
                    for i in 0..n {
                        resd.push(Parameters::InterpreterVector(Box::new(vec_ia[i].clone())));
                    }
                    Parameters::InterpreterVector(Box::new(resd))
                }
            }
        }

        Parameters::Identifier(s) => match ram {
            None => Parameters::Identifier("This variable is not initialized yet".to_string()),
            Some(ref t) => match t.get(s.as_str()) {
                None => Parameters::Null,
                Some(t) => inverse_matrix(&vec![t.clone()], ram),
            },
        },
        _ => Parameters::Null,
    }
}
