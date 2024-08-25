//! # Devide and Conquer
//!
//! Implementations of algorithms based on the devide and conquer paradigm.
use anyhow::Result;
use std::ops::{Add, AddAssign, Mul};

/// Define a square matrix nxn
type SquareMatrix<T> = Vec<Vec<T>>;

pub fn multiply<T>(a: &SquareMatrix<T>, b: &SquareMatrix<T>) -> Result<SquareMatrix<T>>
where
    T: Mul<Output = T> + Add<Output = T> + Copy + Default + AddAssign,
{
    assert_both_square(a, b)?;
    let n = a.len();

    let mut c = vec![vec![T::default(); n]; n];

    for i in 0..n {
        for j in 0..n {
            for (k, &a_val) in a[i].iter().enumerate() {
                c[i][j] += a_val * b[k][j]
            }
        }
    }
    Ok(c)
}

/// Validate the shape of two square matrices
fn assert_both_square<T>(a: &SquareMatrix<T>, b: &SquareMatrix<T>) -> Result<()> {
    let a_row = a.len();
    let a_col = a[0].len();
    let b_row = b.len();
    let b_col = b[0].len();
    if a_row != a_col || b_row != b_col || a_row != b_row {
        return Err(anyhow::anyhow!(
            "Matrices must be square, got shapes {}x{} and {}x{}",
            a_row,
            a_col,
            b_row,
            b_col
        ));
    }
    Ok(())
}

pub fn multiply_conquer<T>(a: &SquareMatrix<T>, b: &SquareMatrix<T>) -> Result<SquareMatrix<T>>
where
    T: Mul<Output = T> + Add<Output = T> + Copy + Default + AddAssign,
{
    assert_both_square(a, b)?;
    let n = a.len();

    if n == 1 {
        return Ok(vec![vec![a[0][0] * b[0][0]]]);
    }

    let mid = n / 2;
    let a11 = split_matrix(a, 0, mid, 0, mid);
    let a12 = split_matrix(a, 0, mid, mid, n);
    let a21 = split_matrix(a, mid, n, 0, mid);
    let a22 = split_matrix(a, mid, n, mid, n);

    let b11 = split_matrix(b, 0, mid, 0, mid);
    let b12 = split_matrix(b, 0, mid, mid, n);
    let b21 = split_matrix(b, mid, n, 0, mid);
    let b22 = split_matrix(b, mid, n, mid, n);

    let c11 = add_matrices(
        &multiply_conquer(&a11, &b11)?,
        &multiply_conquer(&a12, &b21)?,
    );

    let c12 = add_matrices(
        &multiply_conquer(&a11, &b12)?,
        &multiply_conquer(&a12, &b22)?,
    );

    let c21 = add_matrices(
        &multiply_conquer(&a21, &b11)?,
        &multiply_conquer(&a22, &b21)?,
    );

    let c22 = add_matrices(
        &multiply_conquer(&a21, &b12)?,
        &multiply_conquer(&a22, &b22)?,
    );

    let mut c = vec![vec![T::default(); n]; n];

    for i in 0..mid {
        for j in 0..mid {
            c[i][j] = c11[i][j];
            c[i][j + mid] = c12[i][j];
            c[i + mid][j] = c21[i][j];
            c[i + mid][j + mid] = c22[i][j];
        }
    }

    Ok(c)
}

/// Split a matrix into a submatrix
fn split_matrix<T>(
    a: &SquareMatrix<T>,
    start_row: usize,
    end_row: usize,
    start_col: usize,
    end_col: usize,
) -> SquareMatrix<T>
where
    T: Copy + Default,
{
    let rows = end_row - start_row;
    let cols = end_col - start_col;
    let mut result = vec![vec![T::default(); cols]; rows];
    for i in 0..rows {
        for j in 0..cols {
            result[i][j] = a[start_row + i][start_col + j];
        }
    }
    result
}

/// Add two square matrices
fn add_matrices<T>(a: &SquareMatrix<T>, b: &SquareMatrix<T>) -> SquareMatrix<T>
where
    T: Add<Output = T> + Copy + Default + AddAssign,
{
    let n = a.len();
    let mut c = vec![vec![T::default(); n]; n];
    for i in 0..n {
        for j in 0..n {
            c[i][j] = a[i][j] + b[i][j];
        }
    }
    c
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase<T> {
        input: (SquareMatrix<T>, SquareMatrix<T>),
        expected: SquareMatrix<T>,
        description: &'static str,
    }

    fn test_cases_int() -> Vec<TestCase<i32>> {
        vec![
            TestCase {
                input: (vec![vec![1, 2], vec![3, 4]], vec![vec![5, 6], vec![7, 8]]),
                expected: vec![vec![19, 22], vec![43, 50]],
                description: "2x2 matrices",
            },
            TestCase {
                input: (
                    vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
                    vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
                ),
                expected: vec![vec![30, 36, 42], vec![66, 81, 96], vec![102, 126, 150]],
                description: "3x3 matrices",
            },
        ]
    }

    fn test_cases_float() -> Vec<TestCase<f32>> {
        vec![
            TestCase {
                input: (
                    vec![vec![1.0, 2.0], vec![3.0, 4.0]],
                    vec![vec![5.0, 6.0], vec![7.0, 8.0]],
                ),
                expected: vec![vec![19.0, 22.0], vec![43.0, 50.0]],
                description: "2x2 matrices",
            },
            TestCase {
                input: (
                    vec![
                        vec![1.0, 2.0, 3.0],
                        vec![4.0, 5.0, 6.0],
                        vec![7.0, 8.0, 9.0],
                    ],
                    vec![
                        vec![1.0, 2.0, 3.0],
                        vec![4.0, 5.0, 6.0],
                        vec![7.0, 8.0, 9.0],
                    ],
                ),
                expected: vec![
                    vec![30.0, 36.0, 42.0],
                    vec![66.0, 81.0, 96.0],
                    vec![102.0, 126.0, 150.0],
                ],
                description: "3x3 matrices",
            },
        ]
    }

    fn test_cases_wrong_shape() -> Vec<TestCase<i32>> {
        vec![
            TestCase {
                input: (
                    vec![vec![1, 2], vec![3, 4]],
                    vec![vec![1, 2, 3], vec![4, 5, 6]],
                ),
                expected: vec![vec![0]],
                description: "Wrong shape",
            },
            TestCase {
                input: (
                    vec![vec![1, 2, 3], vec![3, 4]],
                    vec![vec![1, 2], vec![3, 4]],
                ),
                expected: vec![vec![0]],
                description: "Not square row a",
            },
            TestCase {
                input: (
                    vec![vec![1, 2], vec![3, 4]],
                    vec![vec![1, 2, 3], vec![3, 4]],
                ),
                expected: vec![vec![0]],
                description: "Not square row b",
            },
            TestCase {
                input: (
                    vec![vec![1, 2], vec![3, 4], vec![5, 6]],
                    vec![vec![1, 2], vec![3, 4]],
                ),
                expected: vec![vec![0]],
                description: "Not square col a",
            },
            TestCase {
                input: (
                    vec![vec![1, 2], vec![3, 4]],
                    vec![vec![1, 2], vec![3, 4], vec![5, 6]],
                ),
                expected: vec![vec![0]],
                description: "Not square col b",
            },
        ]
    }

    #[test]
    fn test_multiply_int() {
        for test_case in test_cases_int().iter() {
            let (a, b) = &test_case.input;
            let result = multiply(a, b);
            assert_eq!(
                result.expect("Error multiplying matrices"),
                test_case.expected,
                "{}",
                test_case.description
            );
        }
    }

    #[test]
    fn test_multiply_float() {
        for test_case in test_cases_float().iter() {
            let (a, b) = &test_case.input;
            let result = multiply(a, b);
            assert_eq!(
                result.expect("Error multiplying matrices"),
                test_case.expected,
                "{}",
                test_case.description
            );
        }
    }

    #[test]
    fn test_multiply_error() {
        for test_case in test_cases_wrong_shape().iter() {
            let (a, b) = &test_case.input;
            let result = multiply(a, b);
            assert!(result.is_err(), "{}", test_case.description);
        }
    }

    #[test]
    fn test_multiply_conquer_int() {
        for test_case in test_cases_int().iter() {
            let (a, b) = &test_case.input;
            if a.len() > 2 {
                continue;
            }
            let result = multiply_conquer(a, b);
            assert_eq!(
                result.expect("Error multiplying matrices"),
                test_case.expected,
                "{}",
                test_case.description
            );
        }
    }

    #[test]
    fn test_multiply_conquer_float() {
        for test_case in test_cases_float().iter() {
            let (a, b) = &test_case.input;
            if a.len() > 2 {
                continue;
            }
            let result = multiply_conquer(a, b);
            assert_eq!(
                result.expect("Error multiplying matrices"),
                test_case.expected,
                "{}",
                test_case.description
            );
        }
    }

    #[test]
    fn test_multiply_conquer_error() {
        for test_case in test_cases_wrong_shape().iter() {
            let (a, b) = &test_case.input;
            let result = multiply_conquer(a, b);
            assert!(result.is_err(), "{}", test_case.description);
        }
    }
}
