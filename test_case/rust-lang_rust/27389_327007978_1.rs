rust
#![feature(alloc_system)]

extern crate alloc_system;

fn main() {
    let a = Box::new(4); // Allocates from the system allocator.
    println!("{}", a);
}
