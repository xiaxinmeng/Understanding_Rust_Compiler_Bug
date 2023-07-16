rust
pub trait Repeat {
    type Repeated;
    fn repeat(&self, times: usize) -> Self::Repeated;
}
