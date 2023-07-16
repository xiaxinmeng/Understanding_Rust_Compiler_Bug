rs
#![feature(inline_const)]
pub fn poc() -> [Option<u8>; 64] { [ const { None }; 64] }
const N: Option<u8> = None;
pub fn poc2() -> [Option<u8>; 64] { [ N; 64] }
