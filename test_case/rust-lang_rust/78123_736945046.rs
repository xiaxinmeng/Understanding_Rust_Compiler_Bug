rust
use std::mem;

#[derive(Debug)]
enum Void {}

fn main() {
    println!("start");

    let x: Void = unsafe { mem::transmute(()) };
    println!("made Void value");

    println!("{:?}", x);
    println!("printed Void value");

    println!("end");
}
