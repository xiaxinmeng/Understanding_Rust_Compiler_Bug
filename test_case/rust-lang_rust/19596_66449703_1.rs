 rust
impl<'a, Args, Result> FnMut<Args, Result> for &'a mut F where F: FnMut<Args, Result> { /* */ }
