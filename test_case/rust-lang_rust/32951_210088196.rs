 rust
fn contains<U: ?Sized>(&self, x: &U) -> bool where T: PartialEq<U> // or `U: PartialEq<T>`
