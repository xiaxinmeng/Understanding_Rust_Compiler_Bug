rust
impl<T : ?Sized> Cell<T> {
    fn read_with<R, F> (self: &'_ Self, f: for<'any> fn(&'any T) -> R) -> R
