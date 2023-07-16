rust
#![feature(lang_items)]
#![no_std]
#![no_main]

#[panic_handler]
fn panic(_panic_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn _start(data: *mut u8, len: usize) -> *const u8 {
    let input = core::slice::from_raw_parts_mut(data, len);
    let input = &mut *input;
    *input.get_mut(3).unwrap_unchecked() = b'a';
    "Hello, World!".as_ptr()
}
