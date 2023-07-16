rust
impl<T: From<U>> TryFrom<U> for T {
    type Error = !;  // <-- or an empty enum before `!` is stable
    fn try_from(u: U) -> Result<Self, Self::Error> {
        Ok(Self::from(u))
    }
}
