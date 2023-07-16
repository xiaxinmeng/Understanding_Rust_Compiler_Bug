rust
#![feature(global_allocator)]
#![feature(allocator_api)]

use std::heap::System;

#[global_allocator]
static ALLOCATOR: System = System;

fn main() {
    println!("Hello, world!");
}
