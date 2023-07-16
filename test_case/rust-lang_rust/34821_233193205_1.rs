 rust
#![feature(asm)]
fn main() {
    let loc = 0;
    unsafe {
        asm!("jmp [$loc]" : : "{loc}"(&loc) : );
    }
}
