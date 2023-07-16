rust
match &self.0 {
    Cow::Borrowed(s) => s,
    Cow::Owned(s) => &*s,
}
