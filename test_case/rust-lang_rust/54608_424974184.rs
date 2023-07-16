rust
#![feature(ptr_internals)]

use core::mem::size_of;
use core::ptr::*;


fn main() {
    println!("size_of::<&u8>={}", size_of::<&u8>());
    println!("size_of::<Option<&u8>>={}", size_of::<Option<&u8>>());
    println!("size_of::<Option<NonNull<u8>>>={}", size_of::<Option<NonNull<u8>>>());
    println!("size_of::<Option<Unique<u8>>>={}", size_of::<Option<Unique<u8>>>());
}
