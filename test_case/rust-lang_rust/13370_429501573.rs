
#![feature(asm)]

fn main() {
    unsafe {
        asm!("xor $1, %rax" :: "r"(0) : "xmm0");
    }
}
