rust
let mut buf : &mut [u8] = init();
{
  let tmp = buf;  // If we automatically add re-borrow here, below line will be failed to compile
  buf = &mut tmp[n..];
}
// Error Message:
// error[E0499]: cannot borrow `*buf` as mutable more than once at a time
