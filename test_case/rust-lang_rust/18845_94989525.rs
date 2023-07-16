 rust
#![feature(box_syntax)]

#![crate_type="lib"]

pub fn test(foo: bool) -> u8 {
    match foo {
        true => *box 0,
        false => 0
    }
}
