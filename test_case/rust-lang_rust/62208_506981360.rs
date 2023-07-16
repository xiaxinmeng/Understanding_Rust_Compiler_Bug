rust
fn take_while_then<F, T, I>(self, while: F, then: T) -> TakeWhileThen<Self, F, T, I>
while
    for<'a> F: FnOnce(&'a Self::Item) -> bool,
    T: FnOnce(Self) -> I,
    I: IntoIterator;
