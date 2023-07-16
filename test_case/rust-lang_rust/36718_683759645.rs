rust
#![feature(asm)]

unsafe fn bkpt() {
    asm!("bkpt aa aaa aa");
}
