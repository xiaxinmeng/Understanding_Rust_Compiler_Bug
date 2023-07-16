rust
pub enum Foo {}
pub trait Food {
    const FOO: Foo;
}
pub trait Bard {
    type Foo: Food;
    fn get_foo() -> Foo { Self::Foo::FOO }
}
