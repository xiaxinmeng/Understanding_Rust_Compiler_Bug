
#![allow(dead_code, unused_imports)]
#![feature(lang_items, core_intrinsics)]
#![feature(custom_attribute)] // <- required for wasm_import_module
#![no_std]
use core::intrinsics;
use core::panic::PanicInfo;

#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    unsafe {
        ::core::intrinsics::abort()
    }
}

mod m1 {
    #[link(wasm_import_module = "m1")]
    extern "C" {
        pub fn f();
    }
    #[link(wasm_import_module = "m1")]
    extern "C" {
        pub fn g();
    }
}

mod m2 {
    #[link(wasm_import_module = "m2")]
    extern "C" {
        pub fn f(_: i32);
    }
}

#[no_mangle]
pub unsafe fn run() {
    m1::f();
    m1::g();
    m2::f(0);
}
