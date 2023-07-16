rust
// since 1.3.0
impl<S: Borrow<str>> Join<&str> for [S] {
    type Output = String;
    …
}
