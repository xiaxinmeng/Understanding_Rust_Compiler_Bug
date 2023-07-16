 rust
#![feature(asm)]

fn main() {
    let mut _test: u32;
    unsafe {
        asm!("mov $2, %eax" : "=a"(_test));
    }
}
