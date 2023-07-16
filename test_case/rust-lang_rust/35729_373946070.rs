
pub trait TryIndex<T>: Index<T> {
    type Error;
    fn try_index(&self, index: T) -> Result<&Self::Output, Self::Error>;
}
pub trait TryIndexMut<T>: IndexMut<T> + TryIndex<T> {
    fn try_index_mut(&mut self, index: T) -> Result<&mut Self::Output, Self::Error>;
}

pub trait UncheckedIndex<T>: Index<T> {
    unsafe fn unchecked_index(&self, index: T) -> &Self::Output;
}
pub trait UncheckedIndexMut<T>: IndexMut<T> + UncheckedIndex<T> {
    fn unchecked_index_mut(&mut self, index: T) -> &mut Self::Output;
}
