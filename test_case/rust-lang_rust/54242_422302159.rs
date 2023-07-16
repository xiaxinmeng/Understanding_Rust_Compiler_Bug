
use std::mem::size_of;

trait Tr {
    type Arr;

    const C: usize = size_of::<u64>();

    fn get_arr() -> Self::Arr;
}

impl Tr for str {
    type Arr = [u8; 8];

    fn get_arr() -> Self::Arr {
        [0; Self::C]
    }
}
