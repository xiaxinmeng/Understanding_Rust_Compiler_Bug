rust
#![feature(existential_type)]

pub trait Foo {
    type Item: std::fmt::Debug;

    fn foo<T: std::fmt::Debug>(_: T) -> Self::Item;
}

#[derive(Debug)]
pub struct S<T>(std::marker::PhantomData<T>);

pub struct S2;

impl Foo for S2 {
    existential type Item: std::fmt::Debug;

    fn foo<T: std::fmt::Debug>(_: T) -> Self::Item {
        S::<T>(Default::default())
    }
}
