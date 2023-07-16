 rust
trait A {
    fn static_method(a: A) -> Self;
    fn method(&self) -> Self;
}

impl A for int {
    fn static_method(a: A) -> int { a + 2 }
    fn method(&self) -> int { *self + 2 }
}

fn tester<T: A>(a: T) {
    let x = a.method();
    let y = T::static_method(a);
}
