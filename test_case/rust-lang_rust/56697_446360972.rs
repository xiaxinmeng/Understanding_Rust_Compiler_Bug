rust
  let px = &mut x as *mut i32 as isize;
  let py = &mut y as *mut i32 as isize;
  let py2 = px + (py - px);
  *(py2 as *mut i32) = 2; // not UB
