rust
pub struct Foo<'a>(&'a i32);

pub trait Bar {
    fn bar(n: &i32) -> Self;
}

impl<'a> Bar for Foo<'a> {
    fn bar(n: &'a i32) -> Foo<'a> {
        Foo(n)
    }
}
