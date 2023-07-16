rust
#![allow(incomplete_features)]
#![feature(specialization)]
use ::core::fmt::Debug;

trait Whatever {
	type Arg: Default + Debug;
	fn print(arg: Self::Arg) {
		println!("{:?}", arg);
	}
	fn print_default() {
		Self::print(Self::Arg::default());
	}
}

#[derive(Default, Debug)]
struct TArg;
impl<T> Whatever for T {
	default type Arg = TArg;
}

#[derive(Default, Debug)]
struct U8Arg;
impl Whatever for u8 {
	type Arg = U8Arg;
}

fn main() {
	// Works
	println!("{:?}", <u8 as Whatever>::Arg::default()); // prints U8Arg
	println!("{:?}", <usize as Whatever>::Arg::default()); // prints TArg
	u8::print_default(); // prints U8Arg
	usize::print_default(); // prints TArg
	u8::print(U8Arg); // prints U8Arg
	// Doesn't work
	usize::print(TArg); // error[E0308]: mismatched types
}

