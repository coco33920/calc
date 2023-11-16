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

/* matrices
* [[a],[x]] merge [[b],[y]] => [[a,b],[x,y]]
*/

/* vectors
* [a] merge [b] => [a,b]
*/
pub fn merge_vectors(vec1: Vec<Parameters>, vec2: Vec<Parameters>) -> Vec<Parameters> {
    let mut result = Vec::new();

    vec1.into_iter().zip(vec2.into_iter()).for_each(|(x, y)| {
        result.push(x);
        result.push(y)
    });

    result
}

pub fn merge_matrices(
    matrix_one: Vec<Vec<Parameters>>,
    matrix_two: Vec<Vec<Parameters>>,
) -> Vec<Vec<Parameters>> {
    let mut result = Vec::new();

    match matrix_one.as_slice() {
        [] => return matrix_two,
        _ => (),
    }

    matrix_one
        .into_iter()
        .zip(matrix_two.into_iter())
        .map(|(x, y)| merge_vectors(x, y))
        .for_each(|x| result.push(x));

    result
}

pub fn transpose(matrix: Vec<Vec<Parameters>>) -> Vec<Vec<Parameters>> {
    let mut transposed_vectors = Vec::new();

    matrix
        .into_iter()
        .map(|x| transpose_vector(x))
        .for_each(|x| transposed_vectors.push(x));

    pub fn aux(
        lst: &[Vec<Vec<Parameters>>],
        mut acc: Vec<Vec<Parameters>>,
    ) -> Vec<Vec<Parameters>> {
        match lst {
            [] => acc,
            [h, q @ ..] => {
                acc = merge_matrices(acc, h.to_vec());
                aux(q, acc)
            }
        }
    }
    aux(transposed_vectors.as_slice(), Vec::new())
}

#[cfg(test)]
mod test {
    use crate::{parsing::ast::Parameters, utils::matrix_utils::merge_matrices};

    use super::{merge_vectors, transpose, transpose_vector};

    #[test]
    fn test_merge_vectors() {
        let mut expected = Vec::new();
        expected.push(Parameters::Int(1));
        expected.push(Parameters::Int(2));

        let mut result1 = Vec::new();
        result1.push(Parameters::Int(1));

        let mut result2 = Vec::new();
        result2.push(Parameters::Int(2));

        let result = merge_vectors(result1, result2);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_easy_transpose() {
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

    #[test]
    fn test_merge_matrices() {
        let expected = vec![
            vec![Parameters::Int(1), Parameters::Float(2.0)],
            vec![Parameters::Int(2), Parameters::Float(1.0)],
        ];
        let _res1 = vec![Parameters::Int(1), Parameters::Int(2)];
        let _res2 = vec![Parameters::Float(2.0), Parameters::Float(1.0)];

        let res1 = transpose_vector(_res1);
        let res2 = transpose_vector(_res2);
        let result = merge_matrices(res1, res2);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_transpose_matrix() {
        let expected = vec![
            vec![Parameters::Int(1), Parameters::Int(2), Parameters::Int(3)],
            vec![
                Parameters::Float(1.0),
                Parameters::Float(2.0),
                Parameters::Float(3.0),
            ],
        ];

        let res1 = vec![
            vec![Parameters::Int(1), Parameters::Float(1.0)],
            vec![Parameters::Int(2), Parameters::Float(2.0)],
            vec![Parameters::Int(3), Parameters::Float(3.0)],
        ];

        let result = transpose(res1);

        assert_eq!(result, expected);
    }
}
