rust
#![no_main]
#![no_std]
#![feature(link_args, lang_items, core_intrinsics)]
#![allow(unused_attributes)]
#![link_args = "--import-memory"]
#![link_args = "--shared"]

#[panic_handler]
#[no_mangle]
pub fn panic(_info: &::core::panic::PanicInfo) -> ! {
    ::core::intrinsics::abort();
}

#[lang = "eh_personality"]
extern fn rust_eh_personality() {}

#[link(wasm_import_module = "import")]
extern { pub fn foo(ptr : *const u8); }

#[export_name = "main"]
pub unsafe fn main() {
    let x = 5u8;
    foo(&x as *const u8);

    let hi: [u8; 2] = [b'h', b'i'];
    foo(hi.as_ptr());
}
