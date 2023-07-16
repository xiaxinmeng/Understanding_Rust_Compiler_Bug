Rust
#![feature(alloc_system, global_allocator, allocator_api)]

extern crate alloc_system;

use alloc_system::System;

#[global_allocator]
static A: System = System;

fn main() {
    let mut data = vec![0; 0x2001];
}

