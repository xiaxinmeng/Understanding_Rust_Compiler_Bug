rust
#![feature(asm)]
fn main() {
    unsafe { asm!("call {}", sym test); }
}
extern fn test() {}
