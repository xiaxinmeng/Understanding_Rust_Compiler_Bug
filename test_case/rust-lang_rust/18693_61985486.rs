 rust
fn two_mult<Scalar, Vector, Matrix>(s: Scalar, v: Vector, m: Matrix) -> (Matrix, Vector)
    where Matrix: Mul<Scalar, Matrix> + Mul<Vector, Vector> {
  (m * s, m * v)
}
