rust
  use std::{
      alloc::{GlobalAlloc, Layout, System},
      ptr,
      sync::atomic::{AtomicBool, Ordering},
  };
  use t_rust_less_lib::memguard::SecretBytes;
  struct Alloc(AtomicBool);
  #[global_allocator]
  static ALLOC: Alloc = Alloc(AtomicBool::new(true));
  // SAFETY: Wraps `System`'s methods, possibly indicating failure.
  unsafe impl GlobalAlloc for Alloc {
      unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
          if self.0.load(Ordering::Relaxed) {
              System.alloc(layout)
          } else {
              ptr::null_mut()
          }
      }
      unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
          System.dealloc(ptr, layout)
      }
  }
  ALLOC.0.store(false, Ordering::Relaxed);
  SecretBytes::with_capacity(0);
  // calls NonNull::new_unchecked() with a null pointer
  // at t_rust_less_lib::memguard::alloc::alloc_aligned()
  