rust
#![feature(unsized_fn_params)]

pub fn take_unsized_slice(s: [u8]) {
    s[0];
}
