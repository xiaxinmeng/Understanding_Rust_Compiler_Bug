rust
#![feature(alloc_system, global_allocator, allocator_api)]

extern crate alloc_system;

#[global_allocator]
static A: alloc_system::System = alloc_system::System;
