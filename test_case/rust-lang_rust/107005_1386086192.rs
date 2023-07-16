rust
#![feature(inline_const_pat)]

let n = 32_usize;
let bug = match n {
    n @ ..= const {u8::MAX as usize} => true,
    _ => false,
};

