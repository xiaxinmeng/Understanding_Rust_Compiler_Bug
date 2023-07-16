 rust
#![feature(unboxed_closures)]

fn foo(x: u32) -> u32 { x }

fn main() {
    let cl = |&: x| foo(x);
    let x = cl.call((42,));
    println!("{}", x);
}
