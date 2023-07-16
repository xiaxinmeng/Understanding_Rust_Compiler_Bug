rust
#![allow(unused_imports, dead_code)]

pub struct S;

pub trait Trait<R> { type Assoc; }

impl<X> Trait<X> for S { type Assoc = X; }

#[repr(C, packed)]
struct Packed {
    pos: Box<<S as Trait<usize>>::Assoc>,
}

fn main() { println!("Hello, world!"); }
