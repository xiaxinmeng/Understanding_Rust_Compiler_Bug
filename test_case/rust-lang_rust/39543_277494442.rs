
#![no_std]
#![feature(const_fn)]
#![feature(lang_items)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![feature(optin_builtin_traits)]

use core::sync::atomic::{AtomicBool, Ordering, ATOMIC_BOOL_INIT};

pub struct UnsafeCell<T: ?Sized> {
    value: T,
}
impl<T> UnsafeCell<T> {
    pub const fn new(value: T) -> UnsafeCell<T> {
        UnsafeCell { value: value }
    }
}

pub struct Mut<T: ?Sized> {
    data: UnsafeCell<T>
}
impl<T> Mut<T> {
    pub const fn new(user_data: T) -> Mut<T>
    {
        Mut
        {
            // data: UnsafeCell { value: user_data }, // no error
            data: UnsafeCell::new(user_data),  // compile error
        }
    }
}
static mut X: UnsafeCell<i32> = UnsafeCell::new(0i32); // no error
static mut Y: Mut<i32> = Mut::new(0i32); // error if use Unsafecell::new()

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
#[lang = "eh_unwind_resume"]
extern "C" fn eh_unwind_resume() {}
#[lang = "panic_fmt"]
extern "C" fn panic_impl(_: core::fmt::Arguments, _: &'static str, _: u32) -> ! {
    loop {}
}

