rust
#![allow(incomplete_features)]
#![feature(const_generics, const_evaluatable_checked)]

pub struct Ref<'a>(&'a i32);

impl<'a> Ref<'a> {
    pub fn foo<const A: usize>()
    where
        ([(); A - 0], ()): Sized,
    {
        Self::foo::<A>()
    }
}
