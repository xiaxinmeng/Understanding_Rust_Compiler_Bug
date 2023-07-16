 rust
impl<T: Clone> MinMaxResult<T> {
    fn into_option(self) -> Option<(T, T)> {
        match self {
            NoElements => None,
            OneElement(x) => Some((x.clone(), x)),
            MinMax(a, b) => Some((a, b))
        }
    }
}
