rust
#[unstable] // We never stabilize this trait
pub trait SlicePattern {
    type Item;
    fn as_slice(&self) -> &[Self::Item];
}

impl<T> SlicePattern for [T] {
    type Item = T;

    fn as_slice(&self) -> &[Self::Item] {
        self
    }
}

impl<T> [T] {
    #[stable]
    pub fn strip_prefix<P: SlicePattern<Item = T>>(&self, prefix: P) -> Option<&[T]>;

    #[stable]
    pub fn strip_suffix<P: SlicePattern<Item = T>>(&self, suffix: P) -> Option<&[T]>;
}
