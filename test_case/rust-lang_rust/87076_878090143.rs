rust
#![feature(const_generics)]

#[derive(PartialEq, Eq)]
pub struct UnitDims {
    pub time: u8,
    pub length: u8,
}

pub struct UnitValue<const DIMS: UnitDims>;

impl<const DIMS: UnitDims> UnitValue<DIMS> {
    fn crash() {}
}

fn main() {
    UnitValue::<{ UnitDims { time: 1, length: 2 } }>::crash();
}
