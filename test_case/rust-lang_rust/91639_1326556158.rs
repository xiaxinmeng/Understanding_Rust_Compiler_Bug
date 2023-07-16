rust
// syntax is arbitrary, just for illustration
#[allow(elide_lifetime)]
pub type DMatrixSlice<'a, T> = MatrixSlice<'a, T, ...>;
