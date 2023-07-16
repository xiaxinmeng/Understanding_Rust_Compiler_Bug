rust
#![feature(extern_types)]
#![allow(dead_code)]

extern "C" {
    pub type Opaque;
}

pub struct TailOpaque {
    a: usize,
    opaque: Opaque,
}

pub fn foo(v: &TailOpaque) -> &Opaque {
    &v.opaque
}
