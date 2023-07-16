 rust
trait Foo<T> {}

impl<T, U> Foo<U> for T where T: Fn(&U) -> bool {}
impl<'a, T> Foo<T> for &'a T {}
