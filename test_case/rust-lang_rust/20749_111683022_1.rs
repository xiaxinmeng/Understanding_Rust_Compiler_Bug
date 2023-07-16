 rust
use std::ops::Index;
impl<M: Memory> Index<u16> for M {
    type Output = u8;
    ...
}
