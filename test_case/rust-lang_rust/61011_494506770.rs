rust
impl<T> MaybeUninit<T> {
 pub unsafe fn from_pointer(ptr: *mut T) -> Option<&mut Self> {
  if ptr.is_null() || !ptr.is_aligned::<T>() {
   return None;
  }
  Some(&mut * (ptr as *mut Self))
 }
}
