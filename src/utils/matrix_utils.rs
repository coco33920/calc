use crate::parsing::ast::Parameters;

pub fn transpose_vector(vector: Vec<Parameters>) -> Vec<Vec<Parameters>> {
    pub fn aux(lst: &[Parameters], mut acc: Vec<Vec<Parameters>>) -> Vec<Vec<Parameters>> {
        match lst {
            [] => acc,
            [h, q @ ..] => {
                let mut vec: Vec<Parameters> = Vec::new();
                vec.push(h.clone());
                acc.push(vec);
                aux(q, acc)
            }
        }
    }

    aux(vector.as_slice(), Vec::new())
}

pub fn transpose(matrix: Vec<Vec<Parameters>>) -> Vec<Vec<Parameters>> {
    pub fn aux(lst: &[Vec<Parameters>], acc: Vec<Vec<Parameters>>) -> Vec<Vec<Parameters>> {
        match lst {
            [] => acc,
            _ => Vec::new(),
        }
    }

    aux(matrix.as_slice(), Vec::new())
}

#[cfg(test)]
mod test {
    use crate::parsing::ast::Parameters;

    use super::transpose_vector;

    #[test]
    fn test_easy() {
        let mut expected = Vec::new();
        let mut a = Vec::new();
        let mut b = Vec::new();
        let mut c = Vec::new();
        a.push(Parameters::Int(1));
        b.push(Parameters::Int(2));
        c.push(Parameters::Int(3));
        expected.push(a);
        expected.push(b);
        expected.push(c);

        let mut vec = Vec::new();
        vec.push(Parameters::Int(1));
        vec.push(Parameters::Int(2));
        vec.push(Parameters::Int(3));

        let result = transpose_vector(vec);
        assert_eq!(result, expected);
    }
}
