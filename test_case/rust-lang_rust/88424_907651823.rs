rust
#![feature(const_trait_impl, const_fn_trait_bound, const_panic)]

pub trait Foo {
    fn bar(self) -> usize;
    const MAX: Self;
}
impl const Foo for usize {
    fn bar(self) -> usize { self }
    const MAX: Self = Self::MAX;
}

const fn spam<T: ~const Foo>(n: usize) {
    assert!(n <= T::MAX.bar());
}

fn main() {}
