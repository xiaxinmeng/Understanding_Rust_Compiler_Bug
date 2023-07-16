rust
#![feature(min_const_generics)]

pub type CellIndex<const D: usize> = [i64; D];
