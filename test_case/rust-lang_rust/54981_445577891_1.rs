rust
#![no_main]
#[no_mangle]
pub unsafe fn main() {
    libc::write(1, b"Hello, world\n\0" as *const u8 as *const libc::c_void, 14);
    libc::_exit(0);
}
