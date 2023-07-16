rust
impl<T: Accum<Output = T>> Accum for Option<T> {
    type Output = Self;

    fn concat(self, other: Self) -> Self::Output {
        match (self, other) {
            (a, None) => a,
            (None, b) => b,
            (Some(a), Some(b)) => Some(a.concat(b)),
        }
    }
}
