Rust
#![allow(incomplete_features)]
#![feature(const_generics, const_evaluatable_checked)]

struct Foo<'a> {
    _r: &'a u8,
}

trait FooTrait {
    const FOO_LEN: usize;

    fn foo_fun(&self, expr: [u8; Self::FOO_LEN]);
}

impl<'a> FooTrait for Foo<'a> {
    const FOO_LEN: usize = 1;

    fn foo_fun(&self, _expr: [u8; 1]) {
        unimplemented!()
    }
}
