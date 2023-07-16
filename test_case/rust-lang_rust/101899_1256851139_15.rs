rust
  // debug-assertions = false
  use refpool::Pool;
  use std::{
      alloc::{GlobalAlloc, Layout, System},
      ptr,
      sync::atomic::{AtomicBool, Ordering},
  };
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
  let pool: Pool<()> = Pool::new(1);
  ALLOC.0.store(false, Ordering::Relaxed);
  pool.fill();
  // calls NonNull::new_unchecked() with a null pointer
  // at <refpool::types::ElementPointer<()> as refpool::pointer::Pointer<refpool::refbox::RefBox<()>>>::wrap()
  // at refpool::pool::Pool::<()>::fill()
  