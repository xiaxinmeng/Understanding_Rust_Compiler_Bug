rust
fn mat_mul<L, R>(dest: &mut D, lhs: &L, rhs: &R)
where
    D: MatrixMut,
    L: Matrix<Scalar = D::Scalar, const ROWS = D::ROWS>,
    R: Matrix<Scalar = D::Scalar, const COLS = D::COLS, const ROWS = L::ROWS>
{
    // ...
}
