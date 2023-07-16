 rust
#![no_std]
#![feature(associated_types, lang_items)]

#[lang = "sized"] pub trait Sized {}

pub trait Deref for Sized? {
    type Sized? Target;

    /// The method called to dereference a value
    fn deref<'a>(&'a self) -> &'a Self::Target;
}
