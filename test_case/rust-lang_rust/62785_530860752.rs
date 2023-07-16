rust
#![no_std]
#![no_main]

#[no_mangle]
pub extern "C" fn efi_main() -> f32 {
    2.0*3.0
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop{}
}
