rust
fn main() {
  let mut x : i32 = 0;
  let mut y : i32 = 1;
  let px = &mut x as *mut i32 as usize;
  let py = px + 4;
  if (py == &mut y as *mut i32 as usize) {
    unsafe {
      *(&mut y as *mut i32 as usize as *mut i32) = 0; // not UB
    }
  }
}
