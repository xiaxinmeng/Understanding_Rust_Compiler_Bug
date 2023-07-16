
$ cat lib.rs
#![feature(lang_items)]
#![no_std]

use core::panic::PanicInfo;

#[lang = "eh_personality"] extern fn eh_personality() {}
#[panic_handler] fn panic_fmt(_info: &PanicInfo) -> ! { loop {} }

pub fn foo() { }

$ rustc lib.rs --crate-type dylib
Undefined symbols for architecture x86_64:
            "_memcmp", referenced from:
...
