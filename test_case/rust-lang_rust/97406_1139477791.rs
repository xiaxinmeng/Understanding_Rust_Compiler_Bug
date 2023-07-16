rust
fn from_fn<F>(_: F) -> impl IntoIterator<Item = u8> { [] }

fn generic_fn<T>() -> Box<dyn Iterator<Item = u8>> {
    Box::new(from_fn(|| {}).into_iter())
    //~^ ERROR the associated type `<impl IntoIterator<Item = u8> as IntoIterator>::IntoIter` may not live long enough
}
