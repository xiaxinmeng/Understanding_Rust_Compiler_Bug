 rust
#![deny(unsafe_code)]

mod inner {
#![allow(unsafe_code)]
thread_local!(pub static KEY: u8 = 1);
}
use inner::KEY;

fn main() {
    KEY.with(|x| { x == &0u8 });
}
