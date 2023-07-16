
#![no_std]
#![no_main]
#![feature(lang_items)]

extern crate etest;

#[lang = "begin_unwind"] extern fn begin_unwind() {}
#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "fail_fmt"] fn fail_fmt() {}
