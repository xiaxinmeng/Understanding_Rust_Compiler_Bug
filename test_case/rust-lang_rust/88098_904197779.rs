rust
#![feature(box_syntax)]
use std::panic::catch_unwind;

struct Foo;

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Foo dropped");
    }
}

fn main() {
    catch_unwind(|| {
        let foo = Foo;
        let a = box [0u8; 1024*1024*1024];
        let b = box [0u8; 1024*1024*1024];
        let c = box [0u8; 1024*1024*1024];
        let d = box [0u8; 1024*1024*1024];
        println!("No oom");
    });
    println!("Caught unwind!");
}
