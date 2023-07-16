rust
fn rfold<B, F>(self, accum: B, f: F) -> B
where
    F: FnMut(B, Self::Item) -> B,
    Self: Sized + DoubleEndedIterator;
