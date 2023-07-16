rust
impl<T: ?Sized> RangeBounds<T> for RangeFull
impl<T: ?Sized, U: Borrow<T>> RangeBounds<T> for RangeFrom<U>
impl<T: ?Sized, U: Borrow<T>> RangeBounds<T> for RangeTo<U>
impl<T: ?Sized, U: Borrow<T>> RangeBounds<T> for Range<U>
impl<T: ?Sized, U: Borrow<T>> RangeBounds<T> for RangeInclusive<U>
impl<T: ?Sized, U: Borrow<T>> RangeBounds<T> for RangeToInclusive<U>
impl<T: ?Sized, U: Borrow<T>> RangeBounds<T> for (Bound<U>, Bound<U>)
