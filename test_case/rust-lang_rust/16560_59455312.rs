 rust
#![feature(unboxed_closures)]

use std::mem;

fn main() {
    let y = 0u8;
    let closure = move |&: x| y + x;

    // Check that both closures are capturing by value
    println!("{}", mem::size_of_val(&closure));  // prints 1

    spawn(proc() {
        let ok = closure;
    })
}
