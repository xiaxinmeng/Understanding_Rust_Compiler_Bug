rust
pub trait Display {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result;

    fn display(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(fmt)
    }
}
