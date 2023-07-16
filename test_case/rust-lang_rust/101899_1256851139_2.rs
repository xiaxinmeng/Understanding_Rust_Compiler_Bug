rust
  use rkyv::AlignedVec;
  AlignedVec::with_capacity(usize::MAX - 14);
  // calls Layout::from_size_align_unchecked(usize::MAX - 14, 16)
  // at rkyv::util::aligned_vec::AlignedVec::with_capacity()
  