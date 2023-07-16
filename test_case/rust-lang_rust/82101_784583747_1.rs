rust
pub fn do_something() {
  if false { unsafe { *(1 as *mut u8); } }
}
