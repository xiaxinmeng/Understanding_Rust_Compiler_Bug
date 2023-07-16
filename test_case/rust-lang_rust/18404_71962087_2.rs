 rust
impl<T: Display + ?Sized> ToString for T {
    fn to_string(&self) -> String {
        match self.as_str_slice() {
            Some(s) => s.to_owned(),
            None => // as before
        }
    }
}
