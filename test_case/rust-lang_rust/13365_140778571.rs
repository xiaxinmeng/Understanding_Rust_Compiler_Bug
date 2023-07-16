 rust
#![feature(asm)]

#[no_mangle]
pub static arr: [u8; 16] = [0; 16];

fn main() {
    unsafe {
        asm!("movups arr, %xmm0" ::: "xmm0");
    }
}
