 rust
#![feature(lang_items, macro_rules)]
#![no_std]
#![crate_type = "staticlib"]

extern crate core;

pub unsafe fn replace<T>(dest: *mut T, mut src: T) -> T {                               |
use core::kinds::Sized;

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }

#[unstable = "may merge with other traits"]
pub trait AsSlice<T> for Sized? {
    fn as_slice<'a>(&'a self) -> &'a [T];
}

#[unstable = "trait is unstable"]
impl<T> AsSlice<T> for [T] {
    #[inline(always)]
    fn as_slice<'a>(&'a self) -> &'a [T] { self }
}

impl<'a, T, Sized? U: AsSlice<T>> AsSlice<T> for &'a U {
    #[inline(always)]
    fn as_slice<'a>(&'a self) -> &'a [T] { AsSlice::as_slice(*self) }
}

impl<'a, T, Sized? U: AsSlice<T>> AsSlice<T> for &'a mut U {
    #[inline(always)]
    fn as_slice<'a>(&'a self) -> &'a [T] { AsSlice::as_slice(*self) }
}

impl<'a> AsSlice<u8> for str {
    #[inline(always)]
    fn as_slice<'a>(&'a self) -> &'a [u8] {
        unsafe { core::mem::transmute(self) }
    }
}
