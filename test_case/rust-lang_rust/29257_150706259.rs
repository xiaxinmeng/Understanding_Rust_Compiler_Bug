
fn f(mut ptr: *const u8) {
  let a = 10u8;
  ptr = &a;
  let b = unsafe { *ptr }; // this dereference is actually safe
}
