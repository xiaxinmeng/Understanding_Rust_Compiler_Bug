 rust
#![feature(unboxed_closures)]

fn main() {
    let x = 0u16;
    let y = 0u16;
    move |:| x + y;
}
