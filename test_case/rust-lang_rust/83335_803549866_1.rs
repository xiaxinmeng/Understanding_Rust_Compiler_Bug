 rust
#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(asm)]

use core::ptr;

global_asm!(r#"
.section .text.boot
.globl _start

_start:
    # read cpu affinity, start core 0, halt rest
    mrs     x19, mpidr_el1
    and     x19, x19, #3
    # compare and branch if non zero
    cbnz    x19, halt

    adrp    x0, _start
    sub     x0, x0, x19, lsl #16
    mov     sp, x0

    b       main_start

halt:
    # unreachable
    wfe
    b       halt
"#);

const UART_BASE: usize = 0x0900_0000;

#[inline]
fn halt() {
    unsafe { asm!("wfi", options(nomem, nostack)) }
}

#[inline]
fn wfe() {
    unsafe { asm!("wfe", options(nomem, nostack)) }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        wfe();
    }
}

extern "C" {
    fn _start();
}

#[no_mangle]
pub unsafe extern "C" fn main_start() {
    let x = _start as usize;
    // just used to prevent `x` is optimized.
    ptr::write_volatile(UART_BASE as *mut u8, (x % 10 + '0' as usize) as u8);

    halt();
}
