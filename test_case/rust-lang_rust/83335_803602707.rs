rust
#![no_std]
#![no_main]
#![feature(global_asm)]

global_asm!(
    r#"
.section .text.boot
.globl _start

_start:
    adrp    x0, _start
    mov     sp, x0

    b       main_start

.section .data
.align 12
data_block:
    .space 0x1000 // 4K
data_block2:
    .space 0x1000 // 4K
"#
);

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern "C" {
    fn _start();
    fn data_block();
    fn data_block2();
}

#[no_mangle]
pub unsafe extern "C" fn main_start() {
    use core::ptr::{read_volatile, write_volatile};
    write_volatile(data_block2 as *mut u8, 42);
    loop {
        read_volatile(data_block as *mut u8);
        read_volatile(data_block2 as *mut u8);
    }
}
