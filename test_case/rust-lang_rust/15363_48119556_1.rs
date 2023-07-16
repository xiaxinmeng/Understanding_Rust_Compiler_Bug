 rust
// foo.rs
#![feature(phase)]

#[phase(plugin)]
extern crate green;

use std::task::TaskBuilder;

green_start!(main)

fn main() {
    for _ in range(0u, 100000) {
        let future = TaskBuilder::new().stack_size(65536).try_future(proc() {});
        drop(future.unwrap());
    }
}
