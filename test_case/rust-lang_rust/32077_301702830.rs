rust
type Matrix2<N> = MatrixN<N, U2>;
type MatrixN<N, D> = MatrixNM<N, D, D>;
type MatrixNM<N, R, C> = Matrix<N, R, C, MatrixArray<N, R, C>>;
pub struct Matrix<N: Scalar, R: Dim, C: Dim, S> { .. }

// Ideally this would be visible in the documentation for Matrix2<N>,
// with the correct types substituted throughout the impl:
impl<N, R: Dim, C: Dim, S> Matrix<N, R, C, S> 
where N: Scalar + ClosedNeg,
        S: StorageMut<N, R, C> { .. }
