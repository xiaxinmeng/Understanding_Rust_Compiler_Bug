rust
#![feature(min_specialization)]


trait Consume<T> {
	fn consume(_: T);
}


//blanket default impl without any specializations
struct Consumer1;

impl<T> Consume<T> for Consumer1 {
	default fn consume(_: T) {
		//...
	}
}


//blanket default impl with 1 specialization
struct Consumer2;

impl<T> Consume<T> for Consumer2 {
	default fn consume(_: T) {
		//...
	}
}

impl Consume<i32> for Consumer2 {
	fn consume(_: i32) {
		//...
	}
}


fn main() {
	Consumer1::consume(true); //ok
	Consumer1::consume(42);   //ok

	Consumer2::consume(true); //error: expected `i32`, found `bool`
	Consumer2::consume(42);   //ok

	//workaround:
	<Consumer2 as Consume<bool>>::consume(true); //ok
}
