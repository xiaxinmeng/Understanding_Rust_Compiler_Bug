 rust
pub enum List<T> {
    Cons(T, ~List<T>),
    Nil,
}

impl<T> List<T> {
    fn prepend<T>(mut self, value: T) {
        self = Cons(value, ~self);
    }
}
