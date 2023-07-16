rust
impl<T> Result<T, io::Error> where T: PartialEq {
    fn compare_kind(&self, other: &Self) -> bool {
        self.map_err(|e| e.kind()) == other.map_err(|e| e.kind())
    }
}
