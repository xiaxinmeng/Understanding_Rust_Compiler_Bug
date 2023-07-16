 rust
#![feature(panic_handler)]
use std::panic;
use std::thread;

fn main() {
    let r1 = thread::spawn(|| {
        panic!("This should print");
    }).join();
    println!("r1: {:?}", r1.is_err());

    let r2 = thread::spawn(|| {
        panic::set_hook(Box::new(|_| ()));
        panic!("This should not print");
    }).join();
    panic::take_hook();
    println!("r2: {:?}", r2.is_err());

    let r3 = thread::spawn(|| {
        panic!("This should print");
    }).join();
    println!("r3: {:?}", r3.is_err());
}

