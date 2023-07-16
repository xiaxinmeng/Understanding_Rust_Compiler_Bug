rust
#![feature(generic_const_exprs)]

trait Foo {
    const N: usize;
    fn foo() -> [u8; Self::N];
}
