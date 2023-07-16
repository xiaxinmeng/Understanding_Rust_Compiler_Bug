rust
#![feature(const_generics)]

struct Struct<T>(T);

impl<T, const N: usize> Struct<[T; N]> {
	fn f() {}
	fn g() { <Struct<[T; N]>>::f(); }
}

fn main() {}
