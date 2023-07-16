 rust
#![no_main]

extern crate corepanic;

#[no_mangle]
pub extern fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    corepanic::do_panic()
}
