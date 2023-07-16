rust
#![feature(nll)]

fn main() {
    let x: (u32, u32);
    x.0 = 1;
    x.1 = 2;
    println!("{:?}", x);
}
