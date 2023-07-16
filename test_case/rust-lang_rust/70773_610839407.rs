rust
#![feature(const_generics)]

const HASH_LEN: usize = 20;
fn init_hash(_: &mut [u8; HASH_LEN]) {}

pub fn foo<'a>() -> &'a () {
    init_hash(&mut [0; HASH_LEN]);
    &()
}
