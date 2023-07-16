rust
#![feature(compiler_builtins_lib)]
#![feature(i128_type)]

extern crate compiler_builtins;

use std::ptr::null_mut;

fn main() {
    let z = compiler_builtins::reimpls::u128_div_mod(3 << 64 | 1, 3 << 64, null_mut());
    assert_eq!(1, z);
}
