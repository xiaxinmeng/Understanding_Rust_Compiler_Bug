rust
#![feature(generic_const_exprs)]

trait Foo {
    const ASSOC: usize;
    fn construct(self) -> [u8; Self::ASSOC];
}

impl<const N: usize> Foo for [u8; N] {
    const ASSOC: usize = N;
    fn construct(self) -> [u8; N] {
        self
    }
}
