
pub trait SliceIndex<T: ?Sized> {
    type Output: ?Sized;
    fn get(self, slice: &T) -> Option<&Self::Output>;
    fn get_mut(self, slice: &mut T) -> Option<&mut Self::Output>;
    unsafe fn get_unchecked(self, slice: &T) -> &Self::Output;
    unsafe fn get_unchecked_mut(self, slice: &mut T) -> &mut Self::Output;
    fn index(self, slice: &T) -> &Self::Output;
    fn index_mut(self, slice: &mut T) -> &mut Self::Output;
}

impl<T> SliceIndex<[T]> for usize { ... }
// etc
impl SliceIndex<str> for RangeFull { ... }
// etc
