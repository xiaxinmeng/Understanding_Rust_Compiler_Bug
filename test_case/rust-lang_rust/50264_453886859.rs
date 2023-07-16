
impl<'a, T> Option<&'a T> {
    fn deref<T>(self) -> Option<&'a T::Target> where T: Deref {
        self.map(|t| &**t)
    }
}
