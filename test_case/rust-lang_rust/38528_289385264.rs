rust
#[cfg(debug_assertions)]
fn test_boxed<T, E, S>(s: S) -> Box<Stream<Item = T, Error = E>>
    where S: Stream<Item = T, Error = E> + 'static
{
    Box::new(s)
}
#[cfg(not(debug_assertions))]
fn test_boxed<S>(s: S) -> S {
    s
}
