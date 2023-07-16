Rust
#![feature(generator_trait)]
#![feature(generators)]
#![feature(unsized_locals)]

use std::ops::Generator;

fn blah() -> impl Generator {
    move || {
        let b: [u8] = *(Box::new([]) as Box<[u8]>);
        
        yield;
        
        for elem in b.iter() {}
    }
}

fn main() {
    blah();
}

