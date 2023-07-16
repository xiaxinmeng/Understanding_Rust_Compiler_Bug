rust
#![feature(lang_items)]

#[lang = "panic_fmt"]
fn panic_fmt() -> ! {
    loop {}
}

fn main() {}
