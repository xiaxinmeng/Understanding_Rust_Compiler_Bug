rust
pub fn foo() -> impl Foo<Bar = impl std::ops::Sub<usize>> {
    5usize
}
