rust
  extern crate rs_libc;
  use std::{
      alloc::{self, Layout},
      mem,
  };
  let size = usize::MAX - mem::size_of::<usize>() - 6;
  // SAFETY: The layout has non-zero size.
  unsafe { alloc::alloc(Layout::from_size_align(size, 8).unwrap()) };
  // calls Layout::from_size_align_unchecked(usize::MAX - 6, 8)
  // at rs_libc::alloc::malloc()
  