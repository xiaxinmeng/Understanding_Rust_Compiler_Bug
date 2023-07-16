rust
// libstd/lib.rs
#![no_std]

#![feature(needs_panic_runtime)]
#![needs_panic_runtime]

#![feature(lang_items)]

#![feature(rustc_attrs)]
#![feature(alloc_error_handler)]
#![feature(allocator_internals)]

#![default_lib_allocator]

extern crate alloc as alloc_crate;

use core::panic::PanicInfo;

#[panic_handler]
pub fn rust_begin_panic(_info: &PanicInfo<'_>) -> ! {
    loop { }
}

pub mod prelude {
    pub mod v1 {
        pub use core::prelude::v1::{Debug};
    }
}

use alloc_crate::alloc::Layout;

#[alloc_error_handler]
pub fn rust_oom(_layout: Layout) -> ! { loop { } }

#[rustc_std_internal_symbol]
pub unsafe extern fn __rdl_alloc(_size: usize, _align: usize) -> *mut u8 { loop { } }

#[rustc_std_internal_symbol]
pub unsafe extern fn __rdl_dealloc(_ptr: *mut u8,
                                   _size: usize,
                                   _align: usize) { loop { } }

#[rustc_std_internal_symbol]
pub unsafe extern fn __rdl_realloc(_ptr: *mut u8,
                                   _old_size: usize,
                                   _align: usize,
                                   _new_size: usize) -> *mut u8 { loop { } }

#[rustc_std_internal_symbol]
pub unsafe extern fn __rdl_alloc_zeroed(_size: usize, _align: usize) -> *mut u8 { loop { } }
