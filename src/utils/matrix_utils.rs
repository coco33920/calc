use std::collections::HashMap;

use crate::{
    interpreting::function::{add, mult},
    parsing::ast::Parameters,
};

pub fn transpose(matrix: Vec<Vec<Parameters>>) -> Vec<Vec<Parameters>> {
    let num_cols = matrix.first().unwrap().len();
    let mut row_iters: Vec<_> = matrix.into_iter().map(Vec::into_iter).collect();
    let mut out: Vec<Vec<_>> = (0..num_cols).map(|_| Vec::new()).collect();

    for out_row in out.iter_mut() {
        for it in row_iters.iter_mut() {
            out_row.push(it.next().unwrap());
        }
    }
    out
}

pub fn mult_matrix(
    a: Vec<Vec<Parameters>>,
    b: Vec<Vec<Parameters>>,
    ram: Option<&HashMap<String, Parameters>>,
) -> Vec<Vec<Parameters>> {
    let first = a.first().unwrap().len();
    let second = b.len();

    if first != second {
        Vec::new()
    } else {
        let n = a.len();
        let p = b.first().unwrap().len();
        let mut res = Vec::new();
        for i in 0..n {
            let mut s = Vec::new();
            for j in 0..p {
                let mut sum: Parameters = Parameters::Null;

                for k in 0..n {
                    let intermediary = mult(
                        a.get(i).unwrap().get(k).unwrap().clone(),
                        b.get(k).unwrap().get(j).unwrap().clone(),
                        ram.as_deref(),
                    );

                    sum = add(sum, intermediary, ram.as_deref())
                }

                s.push(sum);
            }
            res.push(s);
        }

        res
    }
}
