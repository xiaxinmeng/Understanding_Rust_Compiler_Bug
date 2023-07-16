rust
#![feature(const_generics, const_evaluatable_checked)]

pub trait Trait {
    const N: usize;
}

impl<'a> Trait for &'a mut i32 {}

pub fn f<'a>() where [(); <&'a mut i32 as Trait>::N]: {}
