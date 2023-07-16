rust
fn foo(x: &mut NonZeroI32)  {
  let ptr = x as *mut NonZeroI32;
  unsafe { ptr.cast::<i32>().write(0); }
  unsafe { ptr.cast::<i32>().write(1); }
}
