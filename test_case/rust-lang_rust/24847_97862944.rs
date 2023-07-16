 rust
struct DebugHex<T>(T);

impl<T: LowerHex> Debug for DebugHex<T> {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        self.0.fmt(fmt)
    }
}

...
    .field("flag", &DebugHex(&tag))
...
