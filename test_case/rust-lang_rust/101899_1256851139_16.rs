rust
  // serde = { version = "1.0.99", features = ["derive"] }
  use pagecache::{ConfigBuilder, Materializer, PageCache, PAGETABLE_NODE_SZ};
  use serde::{Deserialize, Serialize};
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
          if layout.size() == PAGETABLE_NODE_SZ {
              ptr::null_mut()
          } else {
              System.alloc(layout)
          }
      }
      unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
          System.dealloc(ptr, layout)
      }
  }
  #[derive(Clone, Debug, Deserialize, Serialize)]
  struct Dummy;
  impl Materializer for Dummy {
      fn merge(&mut self, _: &Self) {
          unimplemented!()
      }
  }
  PageCache::<Dummy>::start(ConfigBuilder::new().build()).unwrap();
  // calls Box::from_raw() with a null pointer
  // at pagecache::ds::pagetable::Node1::<
  //     pagecache::ds::stack::Stack<(
  //         core::option::Option<pagecache::pagecache::Update<Dummy>>,
  //         pagecache::pagecache::CacheInfo,
  //     )>,
  // >::new()
  