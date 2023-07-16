
#![feature(const_fn)]
#![feature(lang_items)]
#![feature(libc)]
#![feature(start)]
#![no_main]
extern crate libc;
use std::cell::UnsafeCell;
pub const ABC: UnsafeCell<i32> = UnsafeCell::new(555);
#[no_mangle]
pub extern fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    unsafe { *ABC.get() = 560 };
    println!("ABC = {}", unsafe { *ABC.get() });
    0
}
#[lang = "eh_unwind_resume"]
#[no_mangle]
pub extern fn rust_eh_unwind_resume() {}
