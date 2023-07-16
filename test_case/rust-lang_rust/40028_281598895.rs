rust
impl<T> FromIterator<T> for String where String: Extend<T> {
    fn from_iter<I: Iterator<Item = T>>(iter: I) -> String {
        let mut string = String::new();
        string.extend(iter);
        string
    }
}
