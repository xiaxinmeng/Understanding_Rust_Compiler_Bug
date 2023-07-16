rust
impl<S: Borrow<OsStr>> alloc::slice::Join<&OsStr> for [S] {
    type Output = OsString;
    …
}
