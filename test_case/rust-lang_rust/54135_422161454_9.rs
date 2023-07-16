 rust
// src/main.rs
extern crate bar;

fn main() {
    extern "Rust" {
        fn foo();
    }

    unsafe {
        foo();
    }
}

