 rust
#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, I: Iterator> Iterator for &'a mut I {
    fn all<F>(&mut self, f: F) -> bool
        where Self: Sized, F: FnMut(Self::Item) -> bool
    {
        (*self).all(f)
    }
}
