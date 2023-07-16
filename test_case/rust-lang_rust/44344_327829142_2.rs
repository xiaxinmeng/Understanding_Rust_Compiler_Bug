rust
impl<T> Option<T> {
    …
    pub fn map_or_default<F, U>(self, f: F) -> U
        where F: FnOnce(T) -> U,
              U: Default {

        self.map(f).unwrap_or_default()
    }
    …
}
