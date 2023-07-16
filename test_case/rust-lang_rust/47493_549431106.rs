rust
#![no_std]
#![no_main]

#[panic_handler]
pub fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let a: Result<(), usize> = Err(42);
    a.unwrap();
    loop {}
}
