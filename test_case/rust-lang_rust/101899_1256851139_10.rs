rust
  // overflow-checks = false
  use cryptovec::CryptoVec;
  CryptoVec::with_capacity(usize::MAX / 2 + 2);
  // calls alloc::alloc_zeroed() with size 0
  // at cryptovec::CryptoVec::with_capacity()
  