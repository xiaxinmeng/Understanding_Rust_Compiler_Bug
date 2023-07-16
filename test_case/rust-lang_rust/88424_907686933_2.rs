rust
#![feature(
    const_evaluatable_checked,
    const_fn_trait_bound,
    const_generics,
    const_panic,
    const_trait_impl,
)]

#![allow(incomplete_features)]

pub trait ToFromUsize {
    fn to_usize(self) -> usize;
    fn from_usize(x: usize) -> Self;
    const MAX: Self;
}
impl const ToFromUsize for usize {
    fn to_usize(self) -> usize { self }
    fn from_usize(x: usize) -> Self { x }
    const MAX: Self = Self::MAX;
}

pub const fn assert_nonzero(n: usize) -> usize {
    assert!(n > 0);
    n
}

pub const fn is_contained(n: usize, m: usize) -> usize {
    assert!(n <= m);
    n
}

pub const fn is_representable<Ti: ~const ToFromUsize>(n: usize) -> usize {
    assert!(n <= Ti::MAX.to_usize());
    n
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
#[repr(transparent)]
pub struct Foo<Ti: ToFromUsize + Copy, const N: usize>(Ti)
where [(); assert_nonzero(N)]:,
      [(); is_representable::<Ti>(N - 1)]:;

impl<Ti: ToFromUsize + Copy, const N: usize> Foo<Ti, N>
where [(); assert_nonzero(N)]:,
      [(); is_representable::<Ti>(N - 1)]: {
    pub const fn new(i: Ti) -> Option<Self> {
        if i.to_usize() < N {
            Some(Self(i))
        } else {
            None
        }
    }
}

fn main() {}
