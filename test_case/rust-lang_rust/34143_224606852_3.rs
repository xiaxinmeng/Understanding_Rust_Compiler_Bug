 rust
impl<T> FromIterator<T> for String
    where String: Extend<T>
{
    default fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> String {
        let mut buf = String::new();
        buf.extend(iter);
        buf
    }
}
