rust
impl<T> Option<T> {
    fn and_not<U>(self, other: Option<U>) -> Option<T> {
        match &other {
            &None => self,
            _ => None,
        }
    }
}
