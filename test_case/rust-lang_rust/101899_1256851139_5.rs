rust
  // --target x86_64-unknown-linux-gnu
  use std::{
      ffi::c_void,
      ptr,
      sync::atomic::{AtomicBool, Ordering},
  };
  static ALLOC: AtomicBool = AtomicBool::new(true);
  // SAFETY: Wraps `__libc_malloc()`, possibly indicating failure.
  #[no_mangle]
  extern "C" fn malloc(size: usize) -> *mut c_void {
      extern "C" {
          fn __libc_malloc(size: usize) -> *mut c_void;
      }
      if ALLOC.swap(true, Ordering::Relaxed) {
          unsafe { __libc_malloc(size) }
      } else {
          ptr::null_mut()
      }
  }
  ALLOC.store(false, Ordering::Relaxed);
  // SAFETY: Neither the input nor the output pointer is accessed.
  unsafe {
      miniz_oxide_c_api::tdefl_compress_mem_to_mem(
          &mut () as *mut _ as *mut c_void,
          0,
          ptr::null(),
          0,
          0,
      );
  }
  // calls ptr::write() with a null pointer
  // at miniz_oxide_c_api::tdef::tdefl_compress_mem_to_output()
  