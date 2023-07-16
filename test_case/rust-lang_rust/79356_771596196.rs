rust
#![feature(generic_const_exprs)]

trait Foo {
    const N: usize;
}

struct Num<const N: usize>
where
    [(); <Self as Foo>::N]: ;

impl<const N: usize> Foo for Num<N> {
    const N: usize = N - 1;
}
