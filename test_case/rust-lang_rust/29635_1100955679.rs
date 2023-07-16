rust
#![feature(fundamental)]

pub struct Foo<#[fundamental] S, T> {
    // PhantomData ceremony
}
