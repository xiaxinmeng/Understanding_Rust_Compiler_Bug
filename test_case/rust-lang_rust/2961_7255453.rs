
trait MatrixMultiplyRHS<Result> {
    fn mul(matrix: Matrix) -> Result;
}

impl<RHS:MatrixMultiplyRHS<Result>,Result> Matrix : Mul<RHS, Result> {
    fn mul(rhs: RHS) -> Result {
        rhs.mul(self)
    }
}

impl float : MatrixMultiplyRHS<Matrix> {
    fn mul(matrix: Matrix) -> Matrix {
        // ...implementation of matrix scalar multiply...
    }
}

impl Vector : MatrixMultiplyRHS<Vector> {
    fn mul(matrix: Matrix) -> Vector {
        // ... implementation of matrix multiply for vectors ...
    }
}

impl Matrix : MatrixMultiplyRHS<Matrix> {
    fn mul(matrix: Matrix) -> Matrix {
        // ... implementation of matrix multiply for matrices ...
    }
}
