rust
  // features = ["allocator"]
  // overflow-checks = false
  use rquickjs_core::{Allocator, RustAllocator};
  use std::mem;
  RustAllocator.alloc(usize::MAX - mem::size_of::<usize>() * 2 + 2);
  // calls alloc::alloc() with size 0
  // at <rquickjs_core::allocator::rust::RustAllocator as rquickjs_core::allocator::Allocator>::alloc()
  