rust
impl<T> Foo for [T] {
    // does not require `#![feature(trivial_bounds)]`
    fn foo() -> Self where Self: Sized {
        unreachable!()
    }
    fn bar(&self) {
    }
}
