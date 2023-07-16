
impl<S, I> Future for S
    where S: Stream<Item = I> {
    type Item = Option<(I, S)>;
}
