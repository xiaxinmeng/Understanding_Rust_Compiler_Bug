 rust
#![feature(alloc_jemalloc)]
#![feature(test)]

extern crate alloc_jemalloc;
extern crate test;

fn main() {
    for i in 0..100 {
        let foo = Box::new(i);
        test::black_box(foo);
    }
}
