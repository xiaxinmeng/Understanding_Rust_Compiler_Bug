rust
impl [T] {
    pub const fn get_array<const N: usize>(&self, start_idx: usize) -> Option<&[T; N]>;
    pub const fn get_array_mut<const N: usize>(&mut self, start_idx: usize) -> Option<&mut [T; N]>;
}
