rust
  use std::{
      alloc::{GlobalAlloc, Layout, System},
      ptr,
      sync::atomic::{AtomicBool, Ordering},
  };
  use tract_linalg::{
      frame::{ElementWise, ElementWiseImpl},
      generic::SSigmoid4,
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
  ALLOC.0.store(false, Ordering::Relaxed);
  let ew = ElementWiseImpl::<SSigmoid4, _>::new();
  ew.run(&mut [0.0]).unwrap();
  // calls slice::from_raw_parts_mut() with a null pointer
  // at <tract_linalg::frame::element_wise::ElementWiseImpl<
  //     tract_linalg::generic::sigmoid::SSigmoid4,
  //     f32,
  // > as tract_linalg::frame::element_wise::ElementWise<f32>>::run()
  