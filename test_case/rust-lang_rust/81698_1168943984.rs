rust
#![feature(adt_const_params)]

pub struct SimdU32<const LANES: usize>([u32; LANES]);

impl<const LANES: usize> SimdU32<LANES> {
    pub fn shuffle2<const M: [u32; 2]>(self) -> SimdU32<2> {
        todo!()
    }
}

fn main() {
    let a = SimdU32([2, 4, 1, 9]);
    let c = a.shuffle2::<[3, 1]>();
}
