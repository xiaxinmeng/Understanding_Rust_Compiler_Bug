rust
fn split_first<T>(a: &[T]) -> Option<(&T, &[T])> {
    match a {
        [head, tail @ ..] => Some((head, tail)),
        [] => None,
    }
}
