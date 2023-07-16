rust
impl<T, U> PartialEq<Op<T>> for Op<U>
where
    T: PartialEq<U>,
    U: PartialEq<T>,
{
    #[inline]
    fn eq(&self, other: &Op<T>) -> bool {
        self.as_ref()
            .zip(other.as_ref())
            .map_or(false, |(x, y)| x == y)
    }
}

#[test]
fn derive_partialeq() {
    use std::ffi::OsStr;
    assert_eq!(Op::Some("a"), Op::Some("a"));
    assert_eq!(Op::Some("a"), Op::Some(OsStr::new("a")));
    // assert_ne!(Op::Some("a"), Op::None); // does not compile
    assert_ne!(Op::Some("a"), Op::None::<&str>);
}
