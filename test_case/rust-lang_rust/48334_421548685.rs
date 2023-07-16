rust
#![no_std]
pub fn foo(x: &[u8; 255], y: u8) -> &[u8] {
    &x[..y as usize]
}

#[panic_handler]
fn on_panic_loop(_unused: &core::panic::PanicInfo) -> ! {
    loop {}
}  
