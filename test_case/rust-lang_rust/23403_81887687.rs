
use std::mem;

extern fn test() {
    println!("test()");
}

fn main() {
    println!("{}", mem::size_of::<extern fn()>());

    let a: extern fn() = test;

    let b = a;

    a();
    b();
}
