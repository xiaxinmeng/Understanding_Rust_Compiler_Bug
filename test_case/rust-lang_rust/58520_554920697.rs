rust
pub trait Error: Debug + Display {
    // …
    fn chain(&self) -> Chain<'_>
    where
        Self: Sized + 'static,
    {
        Chain {
            current: Some(self),
        }
    }
    // …
}
