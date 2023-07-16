rust
#![no_main]
#[no_mangle]
pub unsafe fn main() {
    libc::_exit(0);
}
