Rust
fn cut<'a, P: Pattern<'a>>(&'a self, pattern: P) -> Cow<'a, str> {
    let result = self.replace(pattern, "");

    match result == self {
        false => result.into(),
        true => self.into()
    }
}
