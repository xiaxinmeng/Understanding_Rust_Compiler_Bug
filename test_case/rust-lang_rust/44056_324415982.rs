rust
#![feature(lang_items)]
#![feature(start)]
#![feature(libc)]
#![feature(repr_simd)]
#![feature(const_fn)]
#![feature(thread_local)]
#![no_std]
#![no_main]
use core::mem;
extern crate libc;

struct Hoge(u64, u64, u64, u64);

#[thread_local]
static mut STATIC_VAR: Hoge = Hoge(0, 0, 0, 0);

#[no_mangle]
pub extern fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    let mut local_var = Hoge(0, 0, 0, 0);
    unsafe {
        mem::swap(&mut local_var, &mut STATIC_VAR); // CRASH! (sometimes)
        local_var.0 as i32
    }
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }
