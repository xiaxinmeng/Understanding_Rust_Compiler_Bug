 rust
#![crate_type = "bin"]
#![feature(lang_items)]
#![feature(panic_implementation)]
#![no_main]
#![no_std]

use core::panic::PanicInfo;

const Z: () = panic!("cheese");

const Y: () = unreachable!();

const X: () = unimplemented!();

#[lang = "eh_personality"]
fn eh() {}

#[panic_implementation]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
