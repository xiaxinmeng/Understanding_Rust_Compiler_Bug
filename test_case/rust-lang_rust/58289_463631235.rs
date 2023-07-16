rust
pub trait Error: Debug + Display {
…
    fn iter_chain(&self) -> ErrorIter
    where
        Self: Sized + 'static,
    {
        ErrorIter {
            current: Some(self),
        }
    }
…
}
