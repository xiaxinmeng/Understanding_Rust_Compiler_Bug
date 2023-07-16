rust
pub struct Cursor<T> {
    pub fn remaining(&self) -> &[u8];
    pub fn remaining_mut(&mut self) -> &mut [u8];
    pub fn preceding(&self) -> &[u8];
    pub fn preceding_mut(&mut self) -> &mut [u8];
    pub fn split_at(&self) -> (&[u8], &[u8]);
    pub fn split_at_mut(&mut self) -> (&mut [u8], &mut [u8]);
}
