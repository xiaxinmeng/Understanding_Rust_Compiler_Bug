rust
pub struct NestedList<'a>(&'a mut [&'a mut [u8]]);

pub fn readv_at(bufs: &mut [&mut [u8]]) -> NestedList {
    bufs
}
