rust
use std::mem::MaybeUninit;

pub fn foo(thing: MaybeUninit<(u8, u16)>) {
    let bytes: [u8; 4] = unsafe { core::mem::transmute(thing) };
    println!("bytes = {bytes:?}");
}

fn main() {
    foo(MaybeUninit::zeroed())
}
