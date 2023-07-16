rust
fn foo<T>(items: &[T]) -> impl Iterator<Item=(&T, &T)> {
    items
    .iter()
    .enumerate()
    .flat_map(move |(i, x1)| items[i ..].iter().map(move |x2| (x1, x2)))
}
