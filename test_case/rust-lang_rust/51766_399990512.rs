rust
#![crate_type="rlib"]
#![feature(lang_items)]
#![feature(no_core)]
#![no_core]

#[lang = "sized"]
pub trait Sized {}

#[lang = "copy"]
pub trait Copy {}

#[lang = "freeze"]
trait Freeze {}

#[lang = "add"]
pub trait Add<T> {
  #[must_use]
  fn add(self, T) -> Self;
}

impl Add<usize> for usize {
  #[inline]
  fn add(self, other : usize) -> Self { self + other }
}

impl<T> Add<usize> for *const T {
  #[inline]
  fn add(self, off : usize) -> Self {
    ((self as usize) + off) as Self
  }
}

pub unsafe fn buggy(pos : *const u16) -> *const u16 {
  ((pos as usize) + 2) as _
}
