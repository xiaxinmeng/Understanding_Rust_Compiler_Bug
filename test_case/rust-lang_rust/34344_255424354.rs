 rust
#![feature(associated_consts)]

pub trait ArrayTrait {
    const LENGTH: usize;
    fn get_array() -> [i32; Self::LENGTH];
}
