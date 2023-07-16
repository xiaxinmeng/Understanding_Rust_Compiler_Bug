 rust
#![feature(unboxed_closures)]
#![feature(unboxed_closure_sugar)]

fn main() {
    let b = 3u;
    let a = |:| b;
}
