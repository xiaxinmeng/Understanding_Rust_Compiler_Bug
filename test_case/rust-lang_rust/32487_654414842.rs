rust
#![feature(naked_functions, asm)]

#[naked]
pub fn naked(){
    unsafe {
        asm!("ret");
    }
}
