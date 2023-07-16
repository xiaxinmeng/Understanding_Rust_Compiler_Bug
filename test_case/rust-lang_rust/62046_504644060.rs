rust
#![feature(asm)]

fn main() {
    unsafe {
        asm!("nop" : "+r"("r15"));
    }
}
