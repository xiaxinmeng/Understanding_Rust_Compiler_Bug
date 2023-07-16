rust
#![feature(
    const_evaluatable_checked,
    const_generics,
    const_panic,
)]
#![allow(incomplete_features)]

const fn nonzero(n: usize) -> usize {
    assert!(n > 0);
    n
}

fn foo<const N: usize>() -> [u8; nonzero(N)] {
    [0; nonzero(N)]
}

fn main() {
    foo::<5>();
}
