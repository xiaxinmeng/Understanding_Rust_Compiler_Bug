rust
#![feature(type_alias_impl_trait)]

pub struct Foo;

impl std::iter::IntoIterator for Foo {
    type IntoIter = impl Iterator<Item = Self::Item>;
    type Item = ();

    fn into_iter(self) -> Self::IntoIter {
        Some(()).into_iter()
    }
}

pub type FooIntoIter = impl Iterator<Item = ()>;

pub fn foo_into_iter(foo: Foo) -> FooIntoIter {
    foo.into_iter()
}
