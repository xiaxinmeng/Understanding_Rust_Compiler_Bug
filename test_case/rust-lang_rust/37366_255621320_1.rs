 rust
#![crate_type="lib"]
#![feature(asm)]

macro_rules! interrupt_handler {
    () => {
        unsafe fn interrupt_handler() {
            asm!("pop  eax" :::: "intel");
        }
    }
}
interrupt_handler!{}
