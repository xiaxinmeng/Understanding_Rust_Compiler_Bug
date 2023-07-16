rust
fn very_not_okay_function() -> &mut [i32] {
  &mut *ptr::slice_from_raw_parts_mut(0 as *mut _, 42)
}
