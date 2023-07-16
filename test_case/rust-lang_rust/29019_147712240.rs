 rust
fn as_bytes<T: ?Sized>(x: &T) -> &[u8] {
  unsafe {
    std::slice::from_raw_parts(x as *const  T as *const u8, std::mem::size_of_val(x))
  }
}

> as_bytes(&Vec::<()>::new())
[1, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0]

> as_bytes(&Vec::<()>::with_capacity(0))
[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

> as_bytes(&Vec::<()>::with_capacity(1))
[1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
