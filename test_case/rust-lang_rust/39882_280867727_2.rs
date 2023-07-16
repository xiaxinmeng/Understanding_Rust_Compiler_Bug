 rust
#[no_mangle]
pub fn _start() -> ! {
    let mut x = 24;
    unsafe {
        ptr::write(&mut x, 42);
    }

    loop {}
}
