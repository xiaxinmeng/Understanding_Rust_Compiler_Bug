 Rust
// compile with --emit obj and analyize with arm-none-eabi-objdump -s object_file.0.o

#![feature(no_core)]
#![feature(lang_items)]

#![no_core]
#![no_main]

//---------- Minimal Runtime ------------
#[lang = "sized"]
pub trait Sized { }

#[lang = "sync"]
pub trait Sync { }

#[lang = "copy"]
pub trait Copy { }

impl Sync for &'static str { }

//---------- Program Code ---------------
#[allow(dead_code)]
static READ_ONLY_STRING: &'static str = "This is a readonly string";

#[no_mangle]
#[allow(dead_code)]
pub fn function1() { }

#[no_mangle]
#[allow(dead_code)]
pub fn function2() { }
