rust
#[allow_internal_unsafe]
macro_rules! unlikely {
  ($e:expr) => {
    #[allow(unused_unsafe)]
    unsafe {
      std::intrinsics::unlikely($e)
    }
  };
}
