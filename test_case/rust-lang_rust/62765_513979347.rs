rust
fn cross_object_offset<T>(ptr: *const T, n: usize) -> *const T {
  ((ptr as usize) + n*mem::size_of::<T>()) as *const T
}
