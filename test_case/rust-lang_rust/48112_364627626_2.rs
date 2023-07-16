rust
impl<'a, 'b> Backed<'a, *mut String> {
    fn with<F>(self, f: F) -> Backed<'a, ()>
    where
        F: for<'f> FnOnce(&'f mut String) -> (),
    {
        let result = f(self.1.unpack());
        Backed(self.0, result)
    }
}
