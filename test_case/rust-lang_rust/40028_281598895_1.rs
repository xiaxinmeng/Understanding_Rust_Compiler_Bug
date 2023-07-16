rust
// something something monoid
impl<T, C> FromIterator<T> for C where C: Default + Extend<T> {
    fn from_iter<I: Iterator<Item = T>>(iter: I) -> String {
        let mut collection = C::default();
        collection.extend(iter);
        collection
    }
}
