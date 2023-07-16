rust
#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

use core::arch::asm;
pub unsafe fn get_sys_buf() -> *mut u8 {
    let syscall_buf: *mut u8;
    unsafe { asm!("mv {syscall_buff}, tp", syscall_buff = out(reg) syscall_buf) };
    syscall_buf
}
