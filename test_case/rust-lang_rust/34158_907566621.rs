rust
impl<T> Result<T, io::Error> {
    fn kind(self) -> Result<T, io::ErrorKind> {
        self.map_err(|e| e.kind())
    }
}
