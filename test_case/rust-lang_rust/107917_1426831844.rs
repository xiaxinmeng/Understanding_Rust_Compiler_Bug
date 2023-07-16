rust
fn get_shared<T>(ptr: &mut T) -> &UnsafeCell<T> {
  let t = ptr as *mut T as *const UnsafeCell<T>;
  // SAFETY: `T` and `UnsafeCell<T>` have the same memory layout
  unsafe { &*t }
}
