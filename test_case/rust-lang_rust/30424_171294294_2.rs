
fn foo(x: &mut i32) {
  let y = x as *mut _;
  unsafe { *y = 42; }
}
