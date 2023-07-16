rust
impl<T> MaybeUninit<T> {
  pub fn zeroed() -> Self {
    let mut u = MaybeUninit::uninitialized();
    ptr::write_bytes(&mut u as *mut MaybeUninit<T>, 0u8, 1);
    u
  }
}
