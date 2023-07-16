rust
#![feature(generator_trait)]
#![feature(generators)]

use std::ops::Generator;

fn f()-> impl Generator {
    || {
        yield;
    }
}

fn main() {
    std::pin::Pin::new(&mut f()).resume(());
}
