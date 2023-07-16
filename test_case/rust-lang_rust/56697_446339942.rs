rust
fn main() {
  let mut x : i32 = 0;
  let mut y : i32 = 1;
  let px = &mut x as *mut i32 as usize;
  let py = px + 4;
  if (py == &y as *const i32 as usize) {
    unsafe {
      *(py as *mut i32) = 0; // UB
    }
  }
  y
}
