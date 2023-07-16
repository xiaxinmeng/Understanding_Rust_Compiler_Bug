rust
trait Future {
    type Item;
    type Error;
    // This is practically type alias.
    // Can't be changed in trait impl, only used.
    final type Poll = Poll<Self::Item, Self::Error>;

    fn poll(&mut self) -> Self::Poll;
}
