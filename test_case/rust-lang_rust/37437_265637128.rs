
#![feature(asm)]
fn main() {
    unsafe { asm!("" : : "i"("hello")) };
}
