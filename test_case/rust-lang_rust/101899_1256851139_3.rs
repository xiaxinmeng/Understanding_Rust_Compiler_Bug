rust
  // debug-assertions = false
  use sled::Config;
  use std::{
      alloc::{GlobalAlloc, Layout, System},
      ptr,
  };
  struct Alloc;
  #[global_allocator]
  static ALLOC: Alloc = Alloc;
  // SAFETY: Wraps `System`'s methods, possibly indicating failure.
  unsafe impl GlobalAlloc for Alloc {
      unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
          #[cfg(target_pointer_width = "32")]
          const SIZE: usize = 65536;
          #[cfg(target_pointer_width = "64")]
          const SIZE: usize = 4194304;
          if layout.size() == SIZE {
              ptr::null_mut()
          } else {
              System.alloc(layout)
          }
      }
      unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
          System.dealloc(ptr, layout)
      }
  }
  Config::new().open().unwrap();
  // dereferences a null pointer
  // at <sled::pagecache::pagetable::Node1 as crossbeam_epoch::atomic::Pointable>::deref()
  // at crossbeam_epoch::atomic::Shared::<sled::pagecache::pagetable::Node1>::deref()
  // at sled::pagecache::pagetable::PageTable::traverse()
  