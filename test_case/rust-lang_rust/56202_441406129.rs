rust
#![feature(self_struct_ctor)]

use packed_simd::*;

pub trait Bar {
    fn foo(rng: &mut impl rand::Rng) -> Self;
}

pub struct Foo(u8x16);

impl Bar for Foo {
    fn foo(rng: &mut impl rand::Rng) -> Self {
        Self(u8x16::default())
    }
}
