rust
pub trait Foo: Sized {
    fn foo(value: &str) -> Self;
}

impl Foo for &str {
    fn foo(value: &str) -> &str {
        value
    }
}
