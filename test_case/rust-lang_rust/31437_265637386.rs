
#![feature(asm)]
fn main() {
    unsafe { asm!("" : : "X"(main)) };
}
