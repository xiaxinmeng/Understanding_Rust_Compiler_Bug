rust
#![feature(no_core, lang_items)]
#![no_core]

#[lang = "sized"]
trait Sized {}

#[lang = "copy"]
trait Copy {}

fn g<T>(x: T) {}

fn f(x: *mut u8) {
    g(x);
    g(x);
}
