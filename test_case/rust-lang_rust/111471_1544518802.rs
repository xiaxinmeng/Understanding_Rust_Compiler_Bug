rust
#![feature(asm_experimental_arch)]
#![crate_type="lib"]

#[no_mangle]
pub unsafe fn get_global() {
    core::arch::asm!("global.get 0",);
}
