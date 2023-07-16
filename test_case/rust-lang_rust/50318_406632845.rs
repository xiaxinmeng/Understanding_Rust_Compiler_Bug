rust
#![feature(specialization)]
use std::*;

trait Trait {
	type Meta;
	fn hi(&self);
	fn meta(&self) -> Self::Meta;
}

trait Abc {}
trait Def: Abc {}
impl<T> Abc for T {}
impl Def for () {}

impl<T: Abc> Trait for T {
	default type Meta = u8;
	default fn hi(&self) {
	    println!("hello from Abc!")
	}
	default fn meta(&self) -> Self::Meta {
	    unsafe{mem::transmute_copy(&0u8)}
	}
}
impl<T: Abc + Def> Trait for T {
	type Meta = u16;
	fn hi(&self) {
	    println!("hello from Abc+Def!")
	}
	fn meta(&self) -> Self::Meta {
	    0
	}
}

fn main() {
    let a = ();
    a.hi();
    let _: u16 = a.meta();
    
    let b: usize = 123;
    b.hi();
    let _: u8 = b.meta();
}

