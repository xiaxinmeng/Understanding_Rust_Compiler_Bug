
#![feature(unboxed_closures)]
#[allow(unused_variables)]

fn main() {
    let a = 5u;
    let b = move || { a };
}
