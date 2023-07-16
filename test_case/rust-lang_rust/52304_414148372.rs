Rust
#![feature(generators, generator_trait)]
use std::fmt::Debug;

use std::ops::Generator;
fn gen_taker<G: Generator>(mut g: G)
    where G::Yield: Debug, G::Return: Debug
{
    let a = unsafe { g.resume() };
    drop(g);
    println!("{:?}", a);
}
fn main() {
    gen_taker(|| {
        let x = 1;
        yield &x
    });
}
