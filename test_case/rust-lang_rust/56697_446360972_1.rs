rust
  let px = &mut x as *mut i32;
  let py = &mut y as *mut i32;
  let py2 = px.wrapping_offset(py.offset_from(px));
  *py2 = 2; // UB
