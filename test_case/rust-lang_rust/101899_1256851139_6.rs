rust
  // overflow-checks = false
  use arrow::alloc;
  alloc::allocate_aligned::<u16>(usize::MAX / 2 + 1);
  // calls alloc::alloc() with size 0
  // at arrow::alloc::allocate_aligned::<u16>()
  