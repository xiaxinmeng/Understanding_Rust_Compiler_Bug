rust
unsafe {
 if !(x.len() <= x.capacity()) {
  std::hint::unreachable_unchecked();
 }
}
