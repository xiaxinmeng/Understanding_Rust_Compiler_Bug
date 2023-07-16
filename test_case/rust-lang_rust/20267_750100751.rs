rust
#[cfg(crate_type = "staticlib")]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    ...
}
