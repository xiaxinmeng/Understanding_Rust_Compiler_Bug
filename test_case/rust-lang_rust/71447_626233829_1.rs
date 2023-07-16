rust
fn from(cow: Cow<...>) -> Self {
    match cow {
        Cow::Borrowed(v) => Self::from(v),
        Cow::Owned(v) => Self::from(v),
    }
}
