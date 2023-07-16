 rust
#![feature(test)]

extern crate test;

fn main() {
    for i in 0..100 {
        let foo = Box::new(i);
        test::black_box(foo);
    }
}
