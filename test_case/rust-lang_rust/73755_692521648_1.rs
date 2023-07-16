rust
#![feature(lang_items)]
#[lang = "eh_personality"] extern fn eh_personality() {}

#[no_mangle]
pub extern "C" fn magic(a: i32, b: i32) -> (i32, i32) {
    (a + b, a - b)
}
