rust
impl<T> Shared <T> {
  unsafe fn ptr(self) -> *const T
  unsafe fn ptr_mut(self) -> *mut T
  unsafe fn as_ref(&self) -> &T
  unsafe fn as_mut(&mut self) -> &mut T
}

impl<T> Unique <T> {
  unsafe fn ptr(self) -> *mut T
  unsafe fn as_ref(&self) -> &T
  unsafe fn as_mut(&mut self) -> &mut T
}
