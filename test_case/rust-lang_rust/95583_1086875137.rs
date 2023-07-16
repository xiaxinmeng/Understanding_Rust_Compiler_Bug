rust
extern {
  fn store_data(..., data: usize);
  fn register_callback(..., callback: fn(usize)); // gets called with the `data`
}
