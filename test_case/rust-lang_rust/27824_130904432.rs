 rust
#![crate_type = "lib"]
#![feature(lang_items, no_core)]
#![no_core]

fn foo() {
    1_f32 % 2_f32;
}

#[lang = "sized"] trait Sized {}
