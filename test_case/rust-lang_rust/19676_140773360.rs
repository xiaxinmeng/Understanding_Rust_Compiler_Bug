 rust
#![deny(fat_ptr_transmutes)]

use std::mem;

fn main() {
    let x: [u8; 8] = [0; 8];
    let y: [u8; 16] = unsafe {
        mem::transmute(&x[..])
    };
    println!("{:?}", &x[..]);
}
