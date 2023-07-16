rust
#[clippy::weak_invariant(self.start < self.end)]
// alternative framing:
#[clippy::warn_if(self.start >= self.end)]
pub struct Range<T> {
    pub start: T,
    pub end: T,
}
