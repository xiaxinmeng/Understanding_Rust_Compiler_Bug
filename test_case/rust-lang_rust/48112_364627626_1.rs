rust
impl<'a, 'b, U: Unpack<'b>> Backed<'a, U> {
    fn with<F>(self, f: F) -> Backed<'a, ()>
    where
        F: for<'f> FnOnce(<U as Unpack<'f>>::Unpacked) -> (),
    {
        let result = f(self.1.unpack());
        Backed(self.0, result)
    }
}
