
impl<S:Stream> StreamHasher<S> for () {
    fn stream(&self) -> S { fail!() }
}
