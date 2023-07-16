 rust
impl Display for Something {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.message, f)?; // bail early on error
        Display::fmt(&":", f)?; // bail early on error
        Display::fmt(&self.detail, f)?;
    }
}
