 rust
type Result<T> = Result<T, PanicValue>;
struct PanicValue(Box<Any + Send + 'static>);
impl PanicValue {
    fn as_str(&self) -> Option<&str> {
        match self.0.downcast_ref::<&'static str>() {
            Some(s) => Some(*s),
            None => match self.0.downcast_ref::<String>() {
                Some(s) => Some(&s[..]),
                None => None,
            }
        }
    }
}
