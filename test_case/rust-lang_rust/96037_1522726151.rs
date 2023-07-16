rust
#![no_std]
#![feature(naked_functions, asm_experimental_arch, start)]

use core::arch::asm;

#[naked]
#[link_section = ".head.text"]
#[export_name = "head_jump"]
pub unsafe extern "C" fn head_jump() {
    asm!("j {}", sym main, options(noreturn))
}

#[start]
fn start(_: isize, _: *const *const u8) -> isize {
    0
}

fn main() {}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
