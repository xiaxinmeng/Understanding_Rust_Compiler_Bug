rust
#![no_std]

#[derive(Clone, Copy)]
pub struct Slice<'a> {
    pub bytes: &'a [u8],
}
impl<'a> Slice<'a> {
    pub fn into_value(self) -> Slice<'a> {
        self
    }
}
