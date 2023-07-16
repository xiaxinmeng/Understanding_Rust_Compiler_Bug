rust
impl Ord for str {…}
impl<T: ?Sized> Borrow<T> for T {…}
impl<T> RangeBounds<T> for Range<T> {…}

impl<A: ?Sized> Ord for &A where A: Ord {…}
impl<T: ?Sized> Borrow<T> for &T {…}
impl<T /* this part is new: */  : ?Sized> RangeBounds<T> for Range<&T> {…}
