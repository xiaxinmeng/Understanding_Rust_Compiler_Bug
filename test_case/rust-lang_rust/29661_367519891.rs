rust
trait Future {
    type Item;
    type Error;

    fn poll(&mut self) -> FPoll<Self>;
}
type FPoll<F: Future> = Poll<F::Item, F::Error>;
