rust
impl<A> Option<A> {
    fn map<B>(self, transform: impl FnOnce(A) -> B) -> Option<B> {
        match self {
            Self::Some(a) => {
                Option::Some(transform(a))
            }
            Self::None => {
                Option::None
            }
        }
    }
}
