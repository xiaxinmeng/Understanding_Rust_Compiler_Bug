rust
impl<T> Container<T> {
 pub fn deferred_init(uninit: Container<MaybeUninit<T>>, value: T) -> Self {
  uninit.write(value);
  unsafe {
   let disasm = uninit.raw_parts();
   Self::from_raw_parts(disasm)
  }
 }
}
