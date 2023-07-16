rust
  // overflow-checks = false
  use scudo::GlobalScudoAllocator;
  use std::{
      alloc::{GlobalAlloc, Layout},
      mem,
  };
  let size = usize::MAX - mem::size_of::<usize>() * 2 + 2;
  let layout = Layout::array::<u8>(size).unwrap();
  // SAFETY: The layout has non-zero size.
  unsafe { GlobalScudoAllocator.alloc(layout) };
  // calls Layout::from_size_align_unchecked(usize::MAX - size_of::<usize>() * 2 + 2, size_of::<usize>() * 2)
  // at <scudo::GlobalScudoAllocator as core::alloc::global::GlobalAlloc>::alloc()
  