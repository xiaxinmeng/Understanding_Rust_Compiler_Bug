
$ cat lib.rs
#![no_std]
#![feature(lang_items, core_intrinsics)]

#[panic_handler]
#[no_mangle]
pub fn panic(_info: &core::panic::PanicInfo) -> ! {
    core::intrinsics::abort();
}

#[lang = "eh_personality"]
extern "C" fn rust_eh_personality() {}

extern "C" {
    pub fn foo(ptr: *const u8);
}

#[export_name = "_start"]
pub unsafe fn main() {
    foo("hi".as_ptr());
}

$ rustc lib.rs --target=wasm32-unknown-emscripten -O --crate-type=lib -Crelocation-model=pic
