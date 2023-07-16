rust
#![feature(no_core, lang_items)]
#![no_core]

#[lang = "sized"]
trait Sized {}
#[lang = "copy"]
trait Copy {}

pub fn other() -> u32 { return 0xFFFF; }
pub extern fn ext_other() -> u32 { return 0xAAAA; }
