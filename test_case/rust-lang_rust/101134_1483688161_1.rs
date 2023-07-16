rust
#![no_std]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[export_name = "foo"]
pub extern "C" fn foo(a: i32, b: i32) -> i32 {
    a + b // has overflow check in debug build
}
