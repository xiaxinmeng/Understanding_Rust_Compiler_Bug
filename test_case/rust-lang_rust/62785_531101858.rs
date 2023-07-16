rust
#![no_std]
#![no_main]

#[no_mangle]
pub extern "C" fn efi_main() -> i32 {
    0
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop{}
}
