rust
// RUN: --target=aarch64-unknown-none-softfloat -O -Cdebuginfo=0 -Crelocation-model=static
#![no_std]

extern "C" {
    fn _start();
}

#[no_mangle]
pub unsafe extern "C" fn main_start() {
    loop {
        core::ptr::read_volatile(_start as *mut u8);
    }
}
