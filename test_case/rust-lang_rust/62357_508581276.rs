rust
fn read_usize(x: &[u8]) -> usize {
  assert!(x.len() >= mem::size_of::<usize>());
  let ptr = x.as_ptr() as *const usize;
  unsafe { ptr.read_unaligned() }
}
