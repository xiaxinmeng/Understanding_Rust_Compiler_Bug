 rust
#![feature(asm)]
#![feature(core_intrinsics)]

use std::intrinsics::offset;

pub type char_t = i8;
pub type int_t = i32;

pub static mut ARGC: usize = 0;
pub static mut ARGV: *const *const char_t = 0 as *const *const char_t;
pub static mut ENVC: usize = 0;
pub static mut ENVP: *const *const char_t = 0 as *const *const char_t;

extern "C" {
    fn main(argc: int_t,
            argv: *const *const char_t,
            envp: *const *const char_t) -> int_t;
    fn exit(status: int_t) -> !;
}

#[no_mangle]
pub unsafe fn start() {
    asm!("  mov (%rsp), $0
            lea +8(%rsp), $1"
            : "=r"(ARGC), "=r"(ARGV) ::: "volatile");

    ENVP = offset(ARGV, ARGC as isize + 1);

    let mut envc: *const *const char_t = ENVP;
    while (*envc as usize != 0) {
        envc = offset(envc, 1); // increases by one pointer size
    }
    ENVC = (envc as usize - ENVP as usize - 1);

    let status = main(ARGC as int_t, ARGV, ENVP);

    exit(status);
}
