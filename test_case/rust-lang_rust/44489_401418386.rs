rust
#[panic_implementation]
fn panic(info: &core::panic::PanicInfo) -> ! {
    if let Some(s) = info.payload().downcast_ref::<&'static str>() {
        print(s)
    }
    loop {}
}

fn print(_s: &str) {
    // …
}
