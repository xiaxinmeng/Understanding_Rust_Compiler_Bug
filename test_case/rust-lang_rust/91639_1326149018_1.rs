rust
/// Computes the matrix product c <- a * b
fn multiply(c: DMatrixSliceMut<'_, T>, a: DMatrixSlice<'_, T>, b: DMatrixSlice<'_, T>);
