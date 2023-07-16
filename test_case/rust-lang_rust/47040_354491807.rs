 rust
// foo/src/lib.rs
#![feature(lang_items)]
#![no_std]

#[lang = "eh_personality"]
fn eh_personality() {}

#[lang = "panic_fmt"]
fn panic_fmt() {}

#[lang = "start"]
extern "C" fn start(main: fn(), _argc: isize, _argv: *const *const u8) -> isize {
    main();

    0
}
