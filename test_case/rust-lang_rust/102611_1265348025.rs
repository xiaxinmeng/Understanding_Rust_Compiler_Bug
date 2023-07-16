rust
pub struct Foo<'a, A>(&'a A);

impl<'a, A> Foo<'a, A> {
    pub fn foo() {
        Self::foo();
    }
}
