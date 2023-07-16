 rust
impl<
    'a,
    T: HasTransform<'a, Matrix2d>
        + CanTransform<'a, T, Matrix2d>
> RelativeTransform2d<'a> for T {
    fn trans(&'a self, x: Scalar, y: Scalar) -> T {
