rust
  use tinyset::SetU32;
  SetU32::with_capacity_and_bits(usize::MAX / 8 - 2, 0);
  // dereferences a null pointer
  // at tinyset::setu32::SetU32::with_capacity_and_bits()
  