rust
impl<'orig> Pin<&'orig mut T> {
    fn as_mut<'r>(self: &'r mut Pin<&'orig mut T>) -> Pin<&'r mut T>
