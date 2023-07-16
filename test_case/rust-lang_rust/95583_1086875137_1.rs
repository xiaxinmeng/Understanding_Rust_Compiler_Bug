rust
register_callback(..., |addr| {
  let ptr = addr as *mut T;
  ...
});
