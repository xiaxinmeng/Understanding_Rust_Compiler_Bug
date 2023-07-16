rust
  #[no_mangle]
  pub unsafe extern "C" fn getrandom(p: *mut u8, n: usize, _: u32) -> usize {
      p.write_bytes(0, n);
      n
  }
  