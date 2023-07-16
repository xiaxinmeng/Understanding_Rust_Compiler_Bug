rust
impl<T> Result<T, !> {
    pub fn get(self) -> T {
        // can't panic
        self.unwrap()
    }
}
