rust
pub trait Test {}
pub struct Array<T, const N: usize>([T; N]);

impl Test for [u8; 1 + 1] {}
impl Test for Array<u8, {1 + 1}> {}
