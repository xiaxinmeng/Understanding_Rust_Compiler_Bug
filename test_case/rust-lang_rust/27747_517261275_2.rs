rust
impl<S: Borrow<str>> Join<char> for [S] {
    type Output = String;
    fn join(slice: &Self, sep: char) -> String { … }
}
