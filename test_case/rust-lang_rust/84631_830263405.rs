
#![feature(const_generics, const_evaluatable_checked)]

use matrix::matrix::Matrix;

const X: usize = 3;
const Y: usize = 3;

#[test]
fn scalar() {
    let matrix = Matrix::<i32, X, Y>::new([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    let matrix_scalar_double = Matrix::<i32, X, Y>::new([[2, 4, 6], [8, 10, 12], [14, 16, 18]]);
    let matrix_mul = matrix * 2;
    assert_eq!(matrix_mul, matrix_scalar_double);
}

