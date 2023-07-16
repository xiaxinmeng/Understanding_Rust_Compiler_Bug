rust
impl Vec<T> {
    pub fn spare_capacity_mut(&mut self) -> &mut [MaybeUninit<T>];
}
