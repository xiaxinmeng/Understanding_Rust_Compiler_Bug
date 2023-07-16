rust
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]

pub trait AsArray {
    const W: usize;
    fn as_array(&self) -> [f32; Self::W];
}

impl<const N: usize> AsArray for [f32; N] {
    const W: usize = N;

    fn as_array(&self) -> [f32; Self::W] {
        *self
    }
}
