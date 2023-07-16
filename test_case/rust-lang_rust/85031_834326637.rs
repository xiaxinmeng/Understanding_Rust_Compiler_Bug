rust
#![allow(incomplete_features)]
#![feature(const_generics, const_evaluatable_checked)]

pub struct Ref<'a>(&'a i32);

impl<'a> Ref<'a> {
    pub fn foo<const A: usize, const B: usize>()
    where
        ([(); A - B], [(); A - B]): Sized,
    {
        Self::bar::<A, B>()
    }

    pub fn bar<const A: usize, const B: usize>()
    where
        ([(); A - B], [(); A - B]): Sized,
    {
    }
}
