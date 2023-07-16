rust
#[allow_internal_unsafe]
macro_rules! unlikely {
  ($e:expr) => {
    unsafe {
      unlikely($e)
    }
  };
}
