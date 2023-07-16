rust
pub struct FlatMap<I, U: IntoIterator, F>
where I: Iterator, F: FnMut(I::Item) -> U {
    iter: Flatten<Map<I, F>>,
}
