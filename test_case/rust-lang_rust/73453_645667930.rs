rust
#![feature(unsized_tuple_coercion)]

pub type Siz = (u8, [u32; 2]);
pub type Un = (u8, [u32]);

pub fn coercion(x: &Siz) -> &Un {
    x
}
