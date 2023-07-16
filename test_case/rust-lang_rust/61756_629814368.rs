rust
pub trait Captures<U: ?Sized> {}
impl<U: ?Sized, T: ?Sized> Captures<U> for T {}

impl<'a> Transaction<'a> {
    pub fn scan(&mut self) -> impl Iterator<Item = u64> + Captures<&'a Store> + '_ {
        ...
    }
}
