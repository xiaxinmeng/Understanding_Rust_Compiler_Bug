rs
impl<T> fmt::Debug for Delegate<T> {
    fn fmt<'a>(&'a self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Delegate")
            .field(&(self.0 as fn(&'a T)))
            .finish()
    }
}
