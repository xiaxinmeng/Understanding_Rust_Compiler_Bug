rust
#![feature(use_extern_macros)]

extern crate print_input;

fn hello() {
    println!("Hello");
}

macro_rules! print_hello {
    () => {
        print_input::print_input!($crate::hello())
    }
}

fn main() {
    print_hello!();
}
