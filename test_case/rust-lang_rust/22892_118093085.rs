 rust
#![feature(asm)]

fn main() {
    unsafe {
        // - Errors, as expected
        //asm!("lttr %cx" : : "{ecx}" (7*8));
        // No error, no emitted instruction.
        asm!("lttr %cx" : : "{rcx}" (7*8));
    }
}
