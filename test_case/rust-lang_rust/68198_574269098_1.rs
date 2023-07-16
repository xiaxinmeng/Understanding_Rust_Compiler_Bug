rust
impl OnceCell<T> {
    pub fn get_or_init_mut<F>(&mut self, f: F) -> &mut T
}
