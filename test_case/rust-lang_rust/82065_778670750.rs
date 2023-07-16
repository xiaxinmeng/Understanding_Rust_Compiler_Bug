rust
#![feature(llvm_asm)]
fn main() {
    unsafe { llvm_asm!("mov $0, %rax" :: "i"(main as fn())); }
}
