rust
#![feature(no_core, const_trait_impl, rustc_attrs, lang_items, fundamental, auto_traits)]
#![no_std]
#![no_core]
#![crate_type="rlib"]

#[lang = "copy"] trait Copy: Clone { }
#[lang = "clone"] trait Clone: Sized { fn clone(&self) -> Self; }
#[lang = "sized"] trait Sized { }
#[lang = "freeze"] unsafe auto trait Freeze {}
#[lang = "receiver"] trait Receiver { } impl<T: ?Sized> Receiver for &T {}

impl Copy for u8 {}
impl Copy for [u8; 1] {}
impl Clone for u8 { fn clone(&self) -> Self { *self } }
impl Clone for [u8; 1] { fn clone(&self) -> Self { *self } }

struct Pc { _size: [u8; 1], }
impl Copy for Pc {}
impl Clone for Pc { fn clone(&self) -> Pc { *self } }
