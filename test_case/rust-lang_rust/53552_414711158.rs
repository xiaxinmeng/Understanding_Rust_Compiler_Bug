
unsafe impl<T: ?Sized> Freeze for *const T {}
unsafe impl<T: ?Sized> Freeze for *mut T {}
unsafe impl<'a, T: ?Sized> Freeze for &'a T {}
unsafe impl<'a, T: ?Sized> Freeze for &'a mut T {}
