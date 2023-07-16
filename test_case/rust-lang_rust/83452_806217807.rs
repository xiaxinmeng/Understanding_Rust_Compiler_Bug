rust
#![allow(incomplete_features, dead_code)]
#![feature(const_fn, const_trait_impl)]

pub trait Const {
    const ONE: usize = 1usize;
    fn one() -> Self;
}

impl const Const for u8 {
    fn one() -> u8 {
        1u8
    }
}

impl Const for usize {
    fn one() -> usize {
        1usize
    }
}

const fn foo1<T: Const>() -> usize {
    T::ONE
}

const fn foo2<T: Const>() -> T {
    T::one()
}

fn main() {
    // `foo1` explicitly returns `1usize` no matter what, so the `constness` of T's
    // `Const` trait impl is completely irrelevant.
    const X: usize = foo1::<u8>();
    const Y: usize = foo1::<usize>();
    // `u8` did `impl const Const`, so the next line works with `foo2`.
    const Z: u8 = foo2::<u8>();
    // `usize` just did `impl Const`, so the next line does not work with `foo2`.
    const W: usize = foo2::<usize>();
}
