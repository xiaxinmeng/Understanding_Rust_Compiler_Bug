
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    #[cfg(debug_assertions)]
    print_debug!(0, "Error: {}", info);
    #[cfg(not(debug_assertions))]
    print_debug!(0, "Error");
    loop {}
}
