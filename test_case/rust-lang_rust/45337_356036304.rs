rust
#![feature(nll, generators, generator_trait)]
use std::ops::Generator;

fn main() {
	let a = 1;
	let mut generator = unsafe {static || {
	    let b = &a;
	    yield(b); // CRASH the compiler
	    // adding .clone() to b or removing feature(nll) makes the compiler happy 
	}};

	println!("{:?}", generator.resume());
}
