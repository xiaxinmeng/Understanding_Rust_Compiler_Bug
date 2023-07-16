
pub trait TryIndex<T> {
    type Output: ?Sized;
    type Error;
    fn try_index(&self, index: T) -> Result<&Self::Output, Self::Error>;
}
pub trait TryIndexMut<T>: TryIndex<T> {
    fn try_index_mut(&mut self, index: T) -> Result<&mut Self::Output, Self::Error>;
}

impl<T: ?Sized, I> Index<I> for T
where
    T: TryIndex<I>,
{
    type Output = <T as TryIndex<I>>::Output;
    fn index(&self, index: I) -> &<Self as Index<I>>::Output {
        self.try_index(index).unwrap()
    }
}

impl<T: ?Sized, I> IndexMut<I> for T
where
    T: TryIndexMut<I>,
{
    fn index_mut(&mut self, index: I) -> &mut <Self as Index<I>>::Output {
        self.try_index_mut(index).unwrap()
    }
}
