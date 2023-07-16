 rust
#[deriving(PartialOrd, PartialEq, Clone)]
struct NotNan<T> {
    value: T
}
impl<T> NotNan<T> where T: Float {
    fn new(value: T) -> Option<NotNan<T>> { 
        if value.is_nan() { 
            None
        } else { 
            Some(NotNan { value: value }) 
        }
    }
}
impl<T> Eq for NotNan<T> where T: Float {}
impl<T> Ord for NotNan<T> where T: Float {
    fn cmp(&self, other: &NotNan) -> Ordering {
        let &NotNan(a) = self;
        let &NotNan(b) = other;
        if a == b {
            Equal
        } else if a > b {
            Greater
        } else {
            Less
        }
    }
}
