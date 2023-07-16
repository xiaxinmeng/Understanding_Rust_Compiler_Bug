rust
fn okay_function() -> *mut [i32] {
  ptr::slice_from_raw_parts_mut(0 as *mut _, 42)
}

fn very_not_okay_function() -> &mut [i32] {
  slice::from_raw_parts_mut(0 as *mut _, 42)
}
