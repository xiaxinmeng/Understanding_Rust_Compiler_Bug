rust
#[cfg(feature = "some_feature")]
#[no_mangle]
extern "C" fn _start() -> ! {
  setup_code();
  loop {}
}
