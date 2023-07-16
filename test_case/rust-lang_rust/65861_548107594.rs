rust
impl<T> Result<T, !> {
    pub fn get(self) -> T {
        match self {
            Ok(value) => value,
            Err(never) => never
        }
    }
}
