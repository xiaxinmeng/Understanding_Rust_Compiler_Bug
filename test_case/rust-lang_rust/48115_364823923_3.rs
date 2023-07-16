rust
pub struct FlatMap<I, U: IntoIterator, F> {
    iter: Flatten<Map<I, F>, U>,
}
