plain
    Checking rand v0.7.3
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking std v0.0.0 (/checkout/library/std)
    Checking core v0.0.0 (/checkout/library/core)
error: `Layout::dangling` is not yet stable as a const fn
   |
   |
67 |             0 => Ok(NonNull::slice_from_raw_parts(layout.dangling(), 0)),
   |
   = help: add `#![feature(const_alloc_layout)]` to the crate attributes to enable

error: could not compile `alloc` due to previous error
error: could not compile `alloc` due to previous error
warning: build failed, waiting for other jobs to finish...
error: `std::alloc::Layout::dangling` is not yet stable as a const fn
  |
  |
9 |     const DANGLING: NonNull<u8> = LAYOUT.dangling();
  |
  = help: add `#![feature(const_alloc_layout)]` to the crate attributes to enable

error: could not compile `core` due to previous error
